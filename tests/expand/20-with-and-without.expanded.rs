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
impl WithAndWithout {
    #[must_use]
    pub fn with_tls(mut self) -> Self {
        self.tls = true;
        self
    }
    #[must_use]
    pub fn without_tls(mut self) -> Self {
        self.tls = false;
        self
    }
    #[must_use]
    pub fn with_debug_output(mut self) -> Self {
        self.debug_output = true;
        self
    }
    #[must_use]
    pub fn without_debug_output(mut self) -> Self {
        self.debug_output = false;
        self
    }
    #[must_use]
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
    #[must_use]
    pub fn without_name(mut self) -> Self {
        self.name = None;
        self
    }
    #[must_use]
    pub fn with_without_skipped(mut self, without_skipped: Option<usize>) -> Self {
        self.without_skipped = without_skipped;
        self
    }
    #[must_use]
    pub fn with_bool_no_without(mut self, bool_no_without: bool) -> Self {
        self.bool_no_without = bool_no_without;
        self
    }
    #[must_use]
    pub fn with_no_option_set_some(mut self, no_option_set_some: Option<u8>) -> Self {
        self.no_option_set_some = no_option_set_some;
        self
    }
    #[must_use]
    pub fn without_no_option_set_some(mut self) -> Self {
        self.no_option_set_some = None;
        self
    }
}
#[fieldwork(with, without)]
struct MoreEdgeCases {
    #[fieldwork(into)]
    path_with_into: Option<PathBuf>,
    #[fieldwork(rename = custom_name)]
    renamed_field: Option<String>,
    #[fieldwork(with(rename = special_with), without(rename = special_clear))]
    custom_method_names: Option<i32>,
    nested_option: Option<Option<String>>,
    option_vec: Option<Vec<u8>>,
    option_result: Option<Result<String, ()>>,
    #[fieldwork(vis = "pub(crate)")]
    limited_visibility: Option<bool>,
    #[fieldwork(with = false)]
    without_only: Option<String>,
    #[fieldwork(argument = value)]
    custom_arg_name: Option<u64>,
    deref_option: Option<Vec<String>>,
    #[fieldwork(skip)]
    completely_skipped: Option<bool>,
    #[fieldwork(with(skip))]
    with_skipped_without_allowed: Option<String>,
}
impl MoreEdgeCases {
    #[must_use]
    pub fn with_path_with_into(mut self, path_with_into: impl Into<PathBuf>) -> Self {
        self.path_with_into = Some(path_with_into.into());
        self
    }
    #[must_use]
    pub fn without_path_with_into(mut self) -> Self {
        self.path_with_into = None;
        self
    }
    #[must_use]
    pub fn with_custom_name(mut self, custom_name: String) -> Self {
        self.renamed_field = Some(custom_name);
        self
    }
    #[must_use]
    pub fn without_custom_name(mut self) -> Self {
        self.renamed_field = None;
        self
    }
    #[must_use]
    pub fn special_with(mut self, custom_method_names: i32) -> Self {
        self.custom_method_names = Some(custom_method_names);
        self
    }
    #[must_use]
    pub fn special_clear(mut self) -> Self {
        self.custom_method_names = None;
        self
    }
    #[must_use]
    pub fn with_nested_option(mut self, nested_option: Option<String>) -> Self {
        self.nested_option = Some(nested_option);
        self
    }
    #[must_use]
    pub fn without_nested_option(mut self) -> Self {
        self.nested_option = None;
        self
    }
    #[must_use]
    pub fn with_option_vec(mut self, option_vec: Vec<u8>) -> Self {
        self.option_vec = Some(option_vec);
        self
    }
    #[must_use]
    pub fn without_option_vec(mut self) -> Self {
        self.option_vec = None;
        self
    }
    #[must_use]
    pub fn with_option_result(mut self, option_result: Result<String, ()>) -> Self {
        self.option_result = Some(option_result);
        self
    }
    #[must_use]
    pub fn without_option_result(mut self) -> Self {
        self.option_result = None;
        self
    }
    #[must_use]
    pub(crate) fn with_limited_visibility(mut self, limited_visibility: bool) -> Self {
        self.limited_visibility = Some(limited_visibility);
        self
    }
    #[must_use]
    pub(crate) fn without_limited_visibility(mut self) -> Self {
        self.limited_visibility = None;
        self
    }
    #[must_use]
    pub fn without_without_only(mut self) -> Self {
        self.without_only = None;
        self
    }
    #[must_use]
    pub fn with_custom_arg_name(mut self, value: u64) -> Self {
        self.custom_arg_name = Some(value);
        self
    }
    #[must_use]
    pub fn without_custom_arg_name(mut self) -> Self {
        self.custom_arg_name = None;
        self
    }
    #[must_use]
    pub fn with_deref_option(mut self, deref_option: Vec<String>) -> Self {
        self.deref_option = Some(deref_option);
        self
    }
    #[must_use]
    pub fn without_deref_option(mut self) -> Self {
        self.deref_option = None;
        self
    }
    #[must_use]
    pub fn without_with_skipped_without_allowed(mut self) -> Self {
        self.with_skipped_without_allowed = None;
        self
    }
}
#[fieldwork(without)]
struct WithoutIsSkipped {
    string_field: String,
    number_field: u32,
}
impl WithoutIsSkipped {}
#[fieldwork(with, without)]
struct Mixed {
    name: Option<String>,
    active: bool,
    count: u32,
    data: String,
}
impl Mixed {
    #[must_use]
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
    #[must_use]
    pub fn without_name(mut self) -> Self {
        self.name = None;
        self
    }
    #[must_use]
    pub fn with_active(mut self) -> Self {
        self.active = true;
        self
    }
    #[must_use]
    pub fn without_active(mut self) -> Self {
        self.active = false;
        self
    }
    #[must_use]
    pub fn with_count(mut self, count: u32) -> Self {
        self.count = count;
        self
    }
    #[must_use]
    pub fn with_data(mut self, data: String) -> Self {
        self.data = data;
        self
    }
}
