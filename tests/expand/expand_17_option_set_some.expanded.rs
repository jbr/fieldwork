#[fieldwork(set, with, get, get_mut, option_set_some)]
struct OptionBehavior {
    #[fieldwork(deref = "Option<&str>")]
    option_deref: Option<String>,
    #[fieldwork(option_set_some = false)]
    no_option_detection: Option<bool>,
    option_detection: Option<()>,
    #[fieldwork(set(option_set_some = false, option_borrow_inner = false))]
    nothing_fancy_for_set: Option<String>,
}
impl OptionBehavior {
    pub fn option_deref(&self) -> Option<&str> {
        self.option_deref.as_deref()
    }
    pub fn option_deref_mut(&mut self) -> Option<&mut str> {
        self.option_deref.as_deref_mut()
    }
    pub fn set_option_deref(&mut self, option_deref: String) -> &mut Self {
        self.option_deref = Some(option_deref);
        self
    }
    #[must_use]
    pub fn with_option_deref(mut self, option_deref: String) -> Self {
        self.option_deref = Some(option_deref);
        self
    }
    pub fn no_option_detection(&self) -> Option<bool> {
        self.no_option_detection
    }
    pub fn no_option_detection_mut(&mut self) -> Option<&mut bool> {
        self.no_option_detection.as_mut()
    }
    pub fn set_no_option_detection(
        &mut self,
        no_option_detection: Option<bool>,
    ) -> &mut Self {
        self.no_option_detection = no_option_detection;
        self
    }
    #[must_use]
    pub fn with_no_option_detection(
        mut self,
        no_option_detection: Option<bool>,
    ) -> Self {
        self.no_option_detection = no_option_detection;
        self
    }
    pub fn option_detection(&self) -> Option<&()> {
        self.option_detection.as_ref()
    }
    pub fn option_detection_mut(&mut self) -> Option<&mut ()> {
        self.option_detection.as_mut()
    }
    pub fn set_option_detection(&mut self, option_detection: ()) -> &mut Self {
        self.option_detection = Some(option_detection);
        self
    }
    #[must_use]
    pub fn with_option_detection(mut self, option_detection: ()) -> Self {
        self.option_detection = Some(option_detection);
        self
    }
    pub fn nothing_fancy_for_set(&self) -> Option<&str> {
        self.nothing_fancy_for_set.as_deref()
    }
    pub fn nothing_fancy_for_set_mut(&mut self) -> Option<&mut str> {
        self.nothing_fancy_for_set.as_deref_mut()
    }
    pub fn set_nothing_fancy_for_set(
        &mut self,
        nothing_fancy_for_set: Option<String>,
    ) -> &mut Self {
        self.nothing_fancy_for_set = nothing_fancy_for_set;
        self
    }
    #[must_use]
    pub fn with_nothing_fancy_for_set(mut self, nothing_fancy_for_set: String) -> Self {
        self.nothing_fancy_for_set = Some(nothing_fancy_for_set);
        self
    }
}
#[fieldwork(with(option_set_some))]
struct BobTheBuilder {
    string: Option<String>,
    bool: Option<bool>,
}
impl BobTheBuilder {
    #[must_use]
    pub fn with_string(mut self, string: String) -> Self {
        self.string = Some(string);
        self
    }
    #[must_use]
    pub fn with_bool(mut self, bool: bool) -> Self {
        self.bool = Some(bool);
        self
    }
}
#[fieldwork(set, with, get, get_mut, option_set_some)]
struct HandlesNonOptionTypes {
    string: String,
    bool: bool,
}
impl HandlesNonOptionTypes {
    pub fn string(&self) -> &str {
        &*self.string
    }
    pub fn string_mut(&mut self) -> &mut str {
        &mut *self.string
    }
    pub fn set_string(&mut self, string: String) -> &mut Self {
        self.string = string;
        self
    }
    #[must_use]
    pub fn with_string(mut self, string: String) -> Self {
        self.string = string;
        self
    }
    pub fn bool(&self) -> bool {
        self.bool
    }
    pub fn bool_mut(&mut self) -> &mut bool {
        &mut self.bool
    }
    pub fn set_bool(&mut self, bool: bool) -> &mut Self {
        self.bool = bool;
        self
    }
    #[must_use]
    pub fn with_bool(mut self, bool: bool) -> Self {
        self.bool = bool;
        self
    }
}
/// Enum: option_set_some on full-coverage Option fields
#[fieldwork(set, with, get, option_set_some)]
enum EnumWithOptions {
    HasBoth { name: Option<String>, tag: Option<u32> },
    HasName { name: Option<String>, tag: Option<u32> },
}
impl EnumWithOptions {
    pub fn name(&self) -> Option<&str> {
        match self {
            Self::HasBoth { name, .. } | Self::HasName { name, .. } => name.as_deref(),
        }
    }
    pub fn set_name(&mut self, name: String) -> &mut Self {
        match self {
            Self::HasBoth { name: name_binding, .. } => {
                *name_binding = Some(name);
            }
            Self::HasName { name: name_binding, .. } => {
                *name_binding = Some(name);
            }
        }
        self
    }
    #[must_use]
    pub fn with_name(mut self, name: String) -> Self {
        match &mut self {
            Self::HasBoth { name: name_binding, .. } => {
                *name_binding = Some(name);
            }
            Self::HasName { name: name_binding, .. } => {
                *name_binding = Some(name);
            }
        }
        self
    }
    pub fn tag(&self) -> Option<u32> {
        match self {
            Self::HasBoth { tag, .. } | Self::HasName { tag, .. } => *tag,
        }
    }
    pub fn set_tag(&mut self, tag: u32) -> &mut Self {
        match self {
            Self::HasBoth { tag: tag_binding, .. } => {
                *tag_binding = Some(tag);
            }
            Self::HasName { tag: tag_binding, .. } => {
                *tag_binding = Some(tag);
            }
        }
        self
    }
    #[must_use]
    pub fn with_tag(mut self, tag: u32) -> Self {
        match &mut self {
            Self::HasBoth { tag: tag_binding, .. } => {
                *tag_binding = Some(tag);
            }
            Self::HasName { tag: tag_binding, .. } => {
                *tag_binding = Some(tag);
            }
        }
        self
    }
}
/// Enum: option_set_some = false to opt out per field
#[fieldwork(set, option_set_some)]
enum SelectiveOptionSet {
    A {
        #[fieldwork(option_set_some = false)]
        no_some: Option<String>,
        with_some: Option<u32>,
    },
    B { no_some: Option<String>, with_some: Option<u32> },
}
impl SelectiveOptionSet {
    pub fn set_no_some(&mut self, no_some: Option<String>) -> &mut Self {
        match self {
            Self::A { no_some: no_some_binding, .. } => {
                *no_some_binding = no_some;
            }
            Self::B { no_some: no_some_binding, .. } => {
                *no_some_binding = no_some;
            }
        }
        self
    }
    pub fn set_with_some(&mut self, with_some: u32) -> &mut Self {
        match self {
            Self::A { with_some: with_some_binding, .. } => {
                *with_some_binding = Some(with_some);
            }
            Self::B { with_some: with_some_binding, .. } => {
                *with_some_binding = Some(with_some);
            }
        }
        self
    }
}
