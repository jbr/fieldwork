#[fieldwork(get, set, get_mut, with)]
struct AcceptsAnythingInto {
    #[fieldwork(into)]
    string: String,
    #[fieldwork(option_set_some, into)]
    option_string: Option<String>,
}
impl AcceptsAnythingInto {
    pub fn string(&self) -> &str {
        &*self.string
    }
    pub fn string_mut(&mut self) -> &mut str {
        &mut *self.string
    }
    pub fn set_string(&mut self, string: impl Into<String>) -> &mut Self {
        self.string = string.into();
        self
    }
    #[must_use]
    pub fn with_string(mut self, string: impl Into<String>) -> Self {
        self.string = string.into();
        self
    }
    pub fn option_string(&self) -> Option<&str> {
        self.option_string.as_deref()
    }
    pub fn option_string_mut(&mut self) -> Option<&mut str> {
        self.option_string.as_deref_mut()
    }
    pub fn set_option_string(&mut self, option_string: impl Into<String>) -> &mut Self {
        self.option_string = Some(option_string.into());
        self
    }
    #[must_use]
    pub fn with_option_string(mut self, option_string: impl Into<String>) -> Self {
        self.option_string = Some(option_string.into());
        self
    }
}
/// Enum: into on full-coverage field
#[fieldwork(get, set, with)]
enum IntoEnum {
    Alpha { #[fieldwork(into)] name: String, value: i32 },
    Beta { #[fieldwork(into)] name: String, value: i32 },
}
impl IntoEnum {
    pub fn name(&self) -> &str {
        match self {
            Self::Alpha { name, .. } | Self::Beta { name, .. } => &**name,
        }
    }
    pub fn set_name(&mut self, name: impl Into<String>) -> &mut Self {
        match self {
            Self::Alpha { name: name_binding, .. } => {
                *name_binding = name.into();
            }
            Self::Beta { name: name_binding, .. } => {
                *name_binding = name.into();
            }
        }
        self
    }
    #[must_use]
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        match &mut self {
            Self::Alpha { name: name_binding, .. } => {
                *name_binding = name.into();
            }
            Self::Beta { name: name_binding, .. } => {
                *name_binding = name.into();
            }
        }
        self
    }
    pub fn value(&self) -> i32 {
        match self {
            Self::Alpha { value, .. } | Self::Beta { value, .. } => *value,
        }
    }
    pub fn set_value(&mut self, value: i32) -> &mut Self {
        match self {
            Self::Alpha { value: value_binding, .. } => {
                *value_binding = value;
            }
            Self::Beta { value: value_binding, .. } => {
                *value_binding = value;
            }
        }
        self
    }
    #[must_use]
    pub fn with_value(mut self, value: i32) -> Self {
        match &mut self {
            Self::Alpha { value: value_binding, .. } => {
                *value_binding = value;
            }
            Self::Beta { value: value_binding, .. } => {
                *value_binding = value;
            }
        }
        self
    }
}
/// Enum: into on partial-coverage field
#[fieldwork(set)]
enum PartialInto {
    Full { shared: i32, #[fieldwork(into)] label: String },
    Minimal { shared: i32 },
}
impl PartialInto {
    pub fn set_shared(&mut self, shared: i32) -> &mut Self {
        match self {
            Self::Full { shared: shared_binding, .. } => {
                *shared_binding = shared;
            }
            Self::Minimal { shared: shared_binding, .. } => {
                *shared_binding = shared;
            }
        }
        self
    }
}
