use std::path::PathBuf;

#[derive(fieldwork::Fieldwork)]
#[fieldwork(with, without)]
struct WithAndWithout {
    tls: bool,
    debug_output: bool,
    name: Option<String>,

    #[fieldwork(without = false)]
    without_skipped: Option<usize>,

    #[fieldwork(without = false)]
    bool_no_without: bool,

    #[fieldwork(option_set_some = false)]
    no_option_set_some: Option<u8>,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(with, without)]
struct MoreEdgeCases {
    // Test interaction with other features
    #[fieldwork(into)]
    path_with_into: Option<PathBuf>,

    #[fieldwork(rename = custom_name)]
    renamed_field: Option<String>,

    #[fieldwork(with(rename = special_with), without(rename = special_clear))]
    custom_method_names: Option<i32>,

    // Test different Option types
    nested_option: Option<Option<String>>,
    option_vec: Option<Vec<u8>>,
    option_result: Option<Result<String, ()>>,

    // Test with visibility overrides
    #[fieldwork(vis = "pub(crate)")]
    limited_visibility: Option<bool>,

    // Test struct-level vs field-level conflicts
    #[fieldwork(with = false)] // Only without, no with
    without_only: Option<String>,

    // Test with custom argument names
    #[fieldwork(argument = value)]
    custom_arg_name: Option<u64>,

    // Test with deref types
    deref_option: Option<Vec<String>>, // Should this be impl Into<Vec<String>> or impl Into<String>?

    // Test skip interactions
    #[fieldwork(skip)]
    completely_skipped: Option<bool>,

    #[fieldwork(with(skip))] // Skip with but allow without
    with_skipped_without_allowed: Option<String>,
}

// What happens with without on non-Option, non-bool?
#[derive(fieldwork::Fieldwork)]
#[fieldwork(without)]
struct WithoutIsSkipped {
    string_field: String,
    number_field: u32,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(with, without)] // struct-level
struct Mixed {
    name: Option<String>, // gets both with_name(String) and without_name()
    active: bool,         // gets with_active() and without_active()
    count: u32,           // gets only with_count(u32), without silently skipped
    data: String,         // gets only with_data(String), without silently skipped
}

/// Enum: with/without on full-coverage bool and Option fields
/// Bool → no argument; sets to true (with) / false (without)
/// Option → with takes inner T; without sets to None
#[derive(fieldwork::Fieldwork)]
#[fieldwork(with, without)]
enum Toggle {
    On { active: bool, name: Option<String> },
    Off { active: bool, name: Option<String> },
}

/// Enum: without on a partial-coverage Option field → generates with `_ => {}` fallthrough
/// with_token does NOT generate (with requires full coverage)
#[derive(fieldwork::Fieldwork)]
#[fieldwork(with, without)]
enum Connection {
    Authenticated { token: Option<String>, active: bool },
    Anonymous { active: bool },
}
