use quote::ToTokens;
use regex::Regex;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{collections::HashSet, env};
use syn::{
    Attribute, File, Item, ItemEnum, ItemImpl, ItemStruct, ItemTrait, ItemUse, Type, TypePath,
};

#[derive(Debug)]
struct CodeExample {
    input_code: String,
    output_start: usize,
    output_end: usize,
}

#[derive(Debug)]
struct ExtractedCode {
    use_statements: Vec<ItemUse>,
    trait_definitions: Vec<ItemTrait>,
    struct_definitions: Vec<ItemStruct>,
    enum_definitions: Vec<ItemEnum>,
    fieldwork_impls: Vec<ItemImpl>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let verbose = env::args().any(|arg| arg == "--verbose" || arg == "-v");
    let verify = env::args().any(|arg| arg == "--verify");

    let docs_dir = env::args()
        .skip(1)
        .find(|arg| !arg.starts_with("--"))
        .unwrap_or_else(|| "docs".to_string());

    let md_files = find_markdown_files(Path::new(&docs_dir))?;
    println!("Found {} markdown files in {docs_dir}/", md_files.len());

    let example_file = env::current_dir()?.join("examples/docs-expansion.rs");
    let mut any_changed = false;

    for path in &md_files {
        let changed = process_file(path, &example_file, verbose, verify)?;
        if changed {
            any_changed = true;
        }
    }

    if verify && any_changed {
        eprintln!("❌ Documentation is out of date! Run `cargo run --bin docs-gen` to update.");
        std::process::exit(1);
    } else if verify {
        println!("✅ Documentation is up to date.");
    }

    Ok(())
}

fn find_markdown_files(dir: &Path) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut files = Vec::new();
    if !dir.exists() {
        return Ok(files);
    }
    collect_markdown_files(dir, &mut files)?;
    files.sort();
    Ok(files)
}

fn collect_markdown_files(dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_markdown_files(&path, files)?;
        } else if path.extension().and_then(|e| e.to_str()) == Some("md") {
            files.push(path);
        }
    }
    Ok(())
}

fn process_file(
    path: &Path,
    example_file: &Path,
    verbose: bool,
    verify: bool,
) -> Result<bool, Box<dyn Error>> {
    let display = path.display();
    let content = fs::read_to_string(path)?;

    println!("Looking for examples in {display}...");
    let examples = find_expandable_examples(&content)?;

    if examples.is_empty() {
        return Ok(false);
    }

    println!("Found {} examples", examples.len());

    let mut new_content = content.clone();
    let mut updated_count = 0;

    for (i, example) in examples.iter().rev().enumerate() {
        println!("🔄 Processing example {i} of {}...", examples.len());

        match process_example(&example.input_code, example_file) {
            Ok(formatted) => {
                if verbose {
                    println!("Generated output ({} chars):", formatted.len());
                    let lines: Vec<&str> = formatted.lines().collect();
                    for (i, line) in lines.iter().take(5).enumerate() {
                        println!("  {}: {}", i + 1, line);
                    }
                    if lines.len() > 5 {
                        println!("  ... ({} more lines)", lines.len() - 5);
                    }
                }

                let start = example.output_start;
                let end = example.output_end;

                let safe_start = if start >= new_content.len() {
                    new_content.len()
                } else if new_content.is_char_boundary(start) {
                    start
                } else {
                    (0..=start)
                        .rev()
                        .find(|&i| new_content.is_char_boundary(i))
                        .unwrap_or(0)
                };

                let safe_end = if end > new_content.len() {
                    new_content.len()
                } else if new_content.is_char_boundary(end) {
                    end
                } else {
                    (end..new_content.len())
                        .find(|&i| new_content.is_char_boundary(i))
                        .unwrap_or(new_content.len())
                };

                if safe_start <= safe_end {
                    new_content.replace_range(safe_start..safe_end, &formatted);
                } else {
                    eprintln!("⚠️  Invalid range for example {i}, skipping replacement");
                }

                updated_count += 1;
                println!("✅ Example {i} updated successfully");
            }
            Err(e) => {
                eprintln!("❌ Failed to process example {i}: {e}");
                if verbose {
                    eprintln!("Input code was:\n{}", example.input_code);
                }
                continue;
            }
        }
    }

    let changed = new_content != content;

    if !verify && updated_count > 0 {
        fs::write(path, new_content)?;
        println!("📝 Updated {updated_count} examples in {display}");
    }

    Ok(changed)
}

fn find_expandable_examples(content: &str) -> Result<Vec<CodeExample>, Box<dyn Error>> {
    let mut examples = Vec::new();
    let block_pattern = Regex::new(r"(?s)```rust\n(.*?)\n```")?;
    let blocks: Vec<_> = block_pattern.captures_iter(content).collect();

    for (i, block_match) in blocks.iter().enumerate() {
        let block_content = block_match.get(1).unwrap().as_str();

        if block_content.contains("#[derive(") && !block_content.contains("// docgen-skip") {
            let input_code = block_content
                .lines()
                .map(|line| {
                    if let Some(stripped) = line.strip_prefix("# ") {
                        stripped
                    } else if line == "#" {
                        ""
                    } else {
                        line
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");

            if let Some(next_block) = blocks.get(i + 1) {
                let next_full = next_block.get(0).unwrap();
                let output_start = next_full.start() + 8; // +8 for "```rust\n"
                let output_end = next_full.end() - 4; // -4 for "\n```"

                examples.push(CodeExample {
                    input_code,
                    output_start,
                    output_end,
                });
            }
        }
    }

    Ok(examples)
}

fn process_example(input: &str, example_file: &Path) -> Result<String, Box<dyn Error>> {
    let target_items = extract_item_names_from_input(input)?;
    let expanded = expand_single_example(input, example_file)?;
    let extracted = extract_fieldwork_code(&expanded, &target_items)?;
    format_extracted_code(&extracted)
}

fn extract_item_names_from_input(input: &str) -> Result<HashSet<String>, Box<dyn Error>> {
    let parsed: File = syn::parse_str(input)?;
    let mut item_names = HashSet::new();

    for item in parsed.items {
        match item {
            Item::Struct(s) => {
                item_names.insert(s.ident.to_string());
            }
            Item::Enum(e) => {
                item_names.insert(e.ident.to_string());
            }
            _ => {}
        }
    }

    Ok(item_names)
}

fn extract_fieldwork_code(
    expanded: &str,
    target_items: &HashSet<String>,
) -> Result<ExtractedCode, Box<dyn Error>> {
    let parsed: File = syn::parse_str(expanded)?;

    let mut use_statements = vec![];
    let mut trait_definitions = vec![];
    let mut struct_definitions = vec![];
    let mut enum_definitions = vec![];
    let mut fieldwork_impls = vec![];

    for item in parsed.items {
        match item {
            Item::Use(use_item) => {
                use_statements.push(use_item);
            }
            Item::Trait(item_trait) => {
                trait_definitions.push(item_trait);
            }
            Item::Struct(item_struct) => {
                if target_items.contains(&item_struct.ident.to_string()) {
                    struct_definitions.push(item_struct);
                }
            }
            Item::Enum(item_enum) => {
                if target_items.contains(&item_enum.ident.to_string()) {
                    enum_definitions.push(item_enum);
                }
            }
            Item::Impl(item_impl) => {
                if is_fieldwork_impl(&item_impl, target_items) {
                    fieldwork_impls.push(item_impl);
                }
            }
            _ => {}
        }
    }

    Ok(ExtractedCode {
        trait_definitions,
        struct_definitions,
        enum_definitions,
        fieldwork_impls,
        use_statements,
    })
}

fn is_fieldwork_impl(item_impl: &ItemImpl, target_items: &HashSet<String>) -> bool {
    if item_impl.trait_.is_some() {
        return false;
    }

    if let Type::Path(TypePath { path, .. }) = &*item_impl.self_ty {
        if let Some(segment) = path.segments.last() {
            let type_name = segment.ident.to_string();
            return target_items.contains(&type_name);
        }
    }

    false
}

fn format_extracted_code(extracted: &ExtractedCode) -> Result<String, Box<dyn Error>> {
    let mut result = vec!["// GENERATED".to_string()];

    for use_statement in &extracted.use_statements {
        let formatted_use = concise_format(&use_statement.to_token_stream().to_string());
        for line in formatted_use.lines() {
            if !line.trim().is_empty()
                && !line.starts_with("#[prelude_import]")
                && line != "use fieldwork::Fieldwork;"
            {
                result.push(format!("# {line}"));
            }
        }
    }

    for trait_def in &extracted.trait_definitions {
        let formatted_trait = concise_format(&trait_def.to_token_stream().to_string());
        for line in formatted_trait.lines() {
            if !line.trim().is_empty() {
                result.push(format!("# {line}"));
            }
        }
    }

    for struct_def in &extracted.struct_definitions {
        let mut cleaned_struct = struct_def.clone();
        cleaned_struct
            .attrs
            .retain(|attr| !is_fieldwork_attr(attr) && !attr.path().is_ident("doc"));

        for field in &mut cleaned_struct.fields {
            field
                .attrs
                .retain(|attr| !is_fieldwork_attr(attr) && !attr.path().is_ident("doc"));
        }

        let formatted_struct = concise_format(&cleaned_struct.into_token_stream().to_string());
        for line in formatted_struct.lines() {
            if !line.trim().is_empty() {
                result.push(format!("# {line}"));
            }
        }
    }

    for enum_def in &extracted.enum_definitions {
        let mut cleaned_enum = enum_def.clone();
        cleaned_enum
            .attrs
            .retain(|attr| !is_fieldwork_attr(attr) && !attr.path().is_ident("doc"));

        for variant in &mut cleaned_enum.variants {
            variant.attrs.retain(|attr| !is_fieldwork_attr(attr));
            for field in &mut variant.fields {
                field.attrs.retain(|attr| !is_fieldwork_attr(attr));
            }
        }

        let formatted_enum = concise_format(&cleaned_enum.into_token_stream().to_string());
        for line in formatted_enum.lines() {
            if !line.trim().is_empty() {
                result.push(format!("# {line}"));
            }
        }
    }

    for impl_block in &extracted.fieldwork_impls {
        let formatted_impl = prettyplease::unparse(&syn::parse_quote! { #impl_block });
        result.push(formatted_impl);
    }

    Ok(result.join("\n"))
}

fn is_fieldwork_attr(attr: &Attribute) -> bool {
    let path = attr.path();
    path.is_ident("fieldwork") || path.is_ident("field") || path.is_ident("variant")
}

fn concise_format(s: &str) -> String {
    s.replace(" : ", ": ")
        .replace(" < ", "<")
        .replace(" > ", ">")
        .replace(" , ", ", ")
        .replace(" ; ", "; ")
        .replace(" :: ", "::")
        .replace("# ", "#")
        .replace(" ;", ";")
}

fn expand_single_example(input: &str, example_file: &Path) -> Result<String, Box<dyn Error>> {
    let file_content = format!("use fieldwork::Fieldwork;\n\n{input}");
    fs::write(example_file, file_content)?;

    let output = Command::new("cargo")
        .current_dir(env::current_dir()?)
        .args(["expand", "--example", "docs-expansion"])
        .output()?;

    if output.status.success() {
        fs::remove_file(example_file)?;
    } else {
        return Err(format!(
            "cargo expand failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    if output.stdout.is_empty() {
        return Err("cargo expand was empty, that's probably not right".into());
    }

    Ok(String::from_utf8(output.stdout)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_item_names() {
        let input = r#"
        #[derive(fieldwork::Fieldwork)]
        struct User { name: String }

        #[derive(fieldwork::Fieldwork)]
        struct Post { title: String }

        #[derive(fieldwork::Fieldwork)]
        enum Status { Active { name: String }, Inactive { name: String } }
        "#;

        let names = extract_item_names_from_input(input).unwrap();
        assert!(names.contains("User"));
        assert!(names.contains("Post"));
        assert!(names.contains("Status"));
        assert_eq!(names.len(), 3);
    }
}
