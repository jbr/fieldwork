use quote::ToTokens;
use regex::Regex;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::{collections::HashSet, env};
use syn::{File, Item, ItemImpl, ItemStruct, ItemTrait, ItemUse, Type, TypePath};

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
    fieldwork_impls: Vec<ItemImpl>,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Check if we're in verbose mode
    let verbose = env::args().any(|arg| arg == "--verbose" || arg == "-v");
    let verify = env::args().any(|arg| arg == "--verify");

    let docs_path = env::args()
        .skip(1)
        .find(|arg| !arg.starts_with("--"))
        .unwrap_or_else(|| "docs.md".to_string());
    let content = fs::read_to_string(&docs_path)?;

    println!("Looking for examples in {docs_path}...");
    let examples = find_expandable_examples(&content)?;
    println!("Found {} examples", examples.len());

    if verbose {
        for (i, example) in examples.iter().enumerate() {
            println!(
                "Example {}: {} chars of input code",
                i + 1,
                example.input_code.len()
            );
            println!(
                "  First line: {}",
                example.input_code.lines().next().unwrap_or("")
            );
        }
    }

    let mut new_content = content.clone();
    let mut updated_count = 0;

    let example_file = env::current_dir()?.join("examples/docs-expansion.rs");

    // Process examples in reverse order to avoid position shifts
    for (i, example) in examples.iter().rev().enumerate() {
        let example_num = examples.len() - i;

        println!(
            "🔄 Processing example {} of {}...",
            example_num,
            examples.len()
        );

        match process_example(&example.input_code, &example_file) {
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

                // Since we're processing in reverse, original positions should still be valid
                let start = example.output_start;
                let end = example.output_end;

                // Ensure we're on character boundaries and validate range
                let safe_start = if start >= new_content.len() {
                    new_content.len()
                } else if new_content.is_char_boundary(start) {
                    start
                } else {
                    // Find previous char boundary
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
                    // Find next char boundary
                    (end..new_content.len())
                        .find(|&i| new_content.is_char_boundary(i))
                        .unwrap_or(new_content.len())
                };

                // Ensure we have a valid range
                if safe_start <= safe_end {
                    new_content.replace_range(safe_start..safe_end, &formatted);
                } else {
                    eprintln!("⚠️  Invalid range for example {example_num}, skipping replacement");
                }

                updated_count += 1;
                println!("✅ Example {example_num} updated successfully");
            }
            Err(e) => {
                eprintln!("❌ Failed to process example {example_num}: {e}");
                if verbose {
                    eprintln!("Input code was:\n{}", example.input_code);
                }
                continue;
            }
        }
    }

    if verify {
        if new_content != content {
            eprintln!("❌ Documentation is out of date! Run `cargo run --bin docs-gen` to update.");
            std::process::exit(1);
        } else {
            println!("✅ Documentation is up to date.");
        }
    } else {
        // Normal mode: write changes
        if updated_count > 0 {
            fs::write(docs_path, new_content)?;
            println!("📝 Updated {updated_count} examples");
        }
    }
    Ok(())
}

fn find_expandable_examples(content: &str) -> Result<Vec<CodeExample>, Box<dyn Error>> {
    let mut examples = Vec::new();
    let block_pattern = Regex::new(r"(?s)```rust\n(.*?)\n```")?;
    let blocks: Vec<_> = block_pattern.captures_iter(content).collect();

    for (i, block_match) in blocks.iter().enumerate() {
        let block_content = block_match.get(1).unwrap().as_str();

        if block_content.contains("#[derive(") {
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

                // Calculate byte positions for the content inside the next code block
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

// ... rest of your functions remain the same ...
fn process_example(input: &str, example_file: &Path) -> Result<String, Box<dyn Error>> {
    // First, find the struct names in the input to know what we're looking for
    let target_structs = extract_struct_names_from_input(input)?;

    // Expand the code
    let expanded = expand_single_example(input, example_file)?;

    // Parse with syn and extract what we need
    let extracted = extract_fieldwork_code(&expanded, &target_structs)?;

    // Format the output
    format_extracted_code(&extracted)
}

fn extract_struct_names_from_input(input: &str) -> Result<HashSet<String>, Box<dyn Error>> {
    let parsed: File = syn::parse_str(input)?;
    let mut struct_names = HashSet::new();

    for item in parsed.items {
        if let Item::Struct(item_struct) = item {
            struct_names.insert(item_struct.ident.to_string());
        }
    }

    Ok(struct_names)
}

fn extract_fieldwork_code(
    expanded: &str,
    target_structs: &HashSet<String>,
) -> Result<ExtractedCode, Box<dyn Error>> {
    let parsed: File = syn::parse_str(expanded)?;

    let mut use_statements = vec![];
    let mut trait_definitions = vec![];
    let mut struct_definitions = vec![];
    let mut fieldwork_impls = vec![];

    for item in parsed.items {
        match item {
            Item::Use(use_item) => {
                use_statements.push(use_item);
            }
            Item::Trait(item_trait) => {
                // Include all trait definitions found in the expanded code
                trait_definitions.push(item_trait);
            }
            Item::Struct(item_struct) => {
                let struct_name = item_struct.ident.to_string();
                if target_structs.contains(&struct_name) {
                    struct_definitions.push(item_struct);
                }
            }
            Item::Impl(item_impl) => {
                if is_fieldwork_impl(&item_impl, target_structs) {
                    fieldwork_impls.push(item_impl);
                }
            }
            _ => {} // Skip other items (use statements, other impls, etc.)
        }
    }

    Ok(ExtractedCode {
        trait_definitions,
        struct_definitions,
        fieldwork_impls,
        use_statements,
    })
}

fn is_fieldwork_impl(item_impl: &ItemImpl, target_structs: &HashSet<String>) -> bool {
    // Must be an inherent impl (not a trait impl)
    if item_impl.trait_.is_some() {
        return false;
    }

    // Check if this impl is for one of our target structs
    if let Type::Path(TypePath { path, .. }) = &*item_impl.self_ty {
        if let Some(segment) = path.segments.last() {
            let type_name = segment.ident.to_string();
            return target_structs.contains(&type_name);
        }
    }

    false
}

fn format_extracted_code(extracted: &ExtractedCode) -> Result<String, Box<dyn Error>> {
    let mut result = vec!["// GENERATED".to_string()];

    // Add commented trait definitions
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

    // Add commented trait definitions
    for trait_def in &extracted.trait_definitions {
        let formatted_trait = concise_format(&trait_def.to_token_stream().to_string());
        for line in formatted_trait.lines() {
            if !line.trim().is_empty() {
                result.push(format!("# {line}"));
            }
        }
    }

    // Add commented struct definitions (strip fieldwork attributes)
    for struct_def in &extracted.struct_definitions {
        let mut cleaned_struct = struct_def.clone();
        // Remove fieldwork attributes from the struct itself
        cleaned_struct
            .attrs
            .retain(|attr| !attr.path().is_ident("fieldwork") && !attr.path().is_ident("doc"));

        // Remove fieldwork attributes from all fields
        for field in &mut cleaned_struct.fields {
            field
                .attrs
                .retain(|attr| !attr.path().is_ident("fieldwork") && !attr.path().is_ident("doc"));
        }

        let formatted_struct = concise_format(&cleaned_struct.into_token_stream().to_string());
        for line in formatted_struct.lines() {
            if !line.trim().is_empty() {
                result.push(format!("# {line}"));
            }
        }
    }

    // Add fieldwork impl blocks using prettyplease
    for impl_block in &extracted.fieldwork_impls {
        let formatted_impl = prettyplease::unparse(&syn::parse_quote! { #impl_block });
        result.push(formatted_impl);
    }

    Ok(result.join("\n"))
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
    // Write the example code to the .rs file in examples/
    let file_content = format!("use fieldwork::Fieldwork;\n\n{input}");
    fs::write(example_file, file_content)?;

    // Run cargo expand on the example file
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
    fn test_byte_to_char_conversion() {
        let text = "Hello 🦀 World";
        assert_eq!(byte_to_char_pos(text, 0), 0);
        assert_eq!(byte_to_char_pos(text, 6), 6); // Just before 🦀
        assert_eq!(byte_to_char_pos(text, 10), 7); // Just after 🦀
    }

    #[test]
    fn test_extract_struct_names() {
        let input = r#"
        #[derive(fieldwork::Fieldwork)]
        struct User { name: String }
        
        #[derive(fieldwork::Fieldwork)]  
        struct Post { title: String }
        "#;

        let names = extract_struct_names_from_input(input).unwrap();
        assert!(names.contains("User"));
        assert!(names.contains("Post"));
        assert_eq!(names.len(), 2);
    }
}
