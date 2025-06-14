#[fieldwork(get, get_mut)]
struct OptionBehavior {
    #[fieldwork(deref = "Option<&str>")]
    option_deref: Option<String>,
    #[fieldwork(deref = str)]
    option_deref_other_style: Option<String>,
    #[fieldwork(option = false)]
    no_option_detection: Option<String>,
    option_detection: Option<()>,
    #[fieldwork(get_mut(option = false))]
    nothing_fancy_for_get_mut: Option<()>,
}
impl OptionBehavior {
    pub fn option_deref(&self) -> Option<&str> {
        self.option_deref.as_deref()
    }
    pub fn option_deref_mut(&mut self) -> Option<&mut str> {
        self.option_deref.as_deref_mut()
    }
    pub fn option_deref_other_style(&self) -> Option<&str> {
        self.option_deref_other_style.as_deref()
    }
    pub fn option_deref_other_style_mut(&mut self) -> Option<&mut str> {
        self.option_deref_other_style.as_deref_mut()
    }
    pub fn no_option_detection(&self) -> &Option<String> {
        &self.no_option_detection
    }
    pub fn no_option_detection_mut(&mut self) -> &mut Option<String> {
        &mut self.no_option_detection
    }
    pub fn option_detection(&self) -> Option<&()> {
        self.option_detection.as_ref()
    }
    pub fn option_detection_mut(&mut self) -> Option<&mut ()> {
        self.option_detection.as_mut()
    }
    pub fn nothing_fancy_for_get_mut(&self) -> Option<&()> {
        self.nothing_fancy_for_get_mut.as_ref()
    }
    pub fn nothing_fancy_for_get_mut_mut(&mut self) -> &mut Option<()> {
        &mut self.nothing_fancy_for_get_mut
    }
}
#[fieldwork(get, get_mut, option = false)]
struct OptInOption {
    #[fieldwork(deref = "Option<&str>", option = true)]
    option_deref: Option<String>,
    #[fieldwork(deref = str, option = true)]
    option_deref_other_style: Option<String>,
    #[fieldwork(option)]
    option_detection: Option<String>,
    no_option_detection: Option<()>,
    #[fieldwork(get_mut(option = true))]
    option_detection_only_get_mut: Option<()>,
}
impl OptInOption {
    pub fn option_deref(&self) -> Option<&str> {
        self.option_deref.as_deref()
    }
    pub fn option_deref_mut(&mut self) -> Option<&mut str> {
        self.option_deref.as_deref_mut()
    }
    pub fn option_deref_other_style(&self) -> Option<&str> {
        self.option_deref_other_style.as_deref()
    }
    pub fn option_deref_other_style_mut(&mut self) -> Option<&mut str> {
        self.option_deref_other_style.as_deref_mut()
    }
    pub fn option_detection(&self) -> Option<&String> {
        self.option_detection.as_ref()
    }
    pub fn option_detection_mut(&mut self) -> Option<&mut String> {
        self.option_detection.as_mut()
    }
    pub fn no_option_detection(&self) -> &Option<()> {
        &self.no_option_detection
    }
    pub fn no_option_detection_mut(&mut self) -> &mut Option<()> {
        &mut self.no_option_detection
    }
    pub fn option_detection_only_get_mut(&self) -> &Option<()> {
        &self.option_detection_only_get_mut
    }
    pub fn option_detection_only_get_mut_mut(&mut self) -> Option<&mut ()> {
        self.option_detection_only_get_mut.as_mut()
    }
}
#[fieldwork(get(option = true), get_mut, option = false)]
struct OptionOnlyForGet {
    #[fieldwork(get(option = false))]
    no_option_detection: Option<()>,
    #[fieldwork(get(deref = str))]
    option_deref: Option<String>,
    #[fieldwork(option = true)]
    field_overrides: Option<String>,
    option_only_for_get: Option<()>,
    #[fieldwork(get_mut(option), get(option = false))]
    option_detection_only_get_mut: Option<()>,
}
impl OptionOnlyForGet {
    pub fn no_option_detection(&self) -> &Option<()> {
        &self.no_option_detection
    }
    pub fn no_option_detection_mut(&mut self) -> &mut Option<()> {
        &mut self.no_option_detection
    }
    pub fn option_deref(&self) -> Option<&str> {
        self.option_deref.as_deref()
    }
    pub fn option_deref_mut(&mut self) -> &mut Option<String> {
        &mut self.option_deref
    }
    pub fn field_overrides(&self) -> Option<&String> {
        self.field_overrides.as_ref()
    }
    pub fn field_overrides_mut(&mut self) -> Option<&mut String> {
        self.field_overrides.as_mut()
    }
    pub fn option_only_for_get(&self) -> Option<&()> {
        self.option_only_for_get.as_ref()
    }
    pub fn option_only_for_get_mut(&mut self) -> &mut Option<()> {
        &mut self.option_only_for_get
    }
    pub fn option_detection_only_get_mut(&self) -> &Option<()> {
        &self.option_detection_only_get_mut
    }
    pub fn option_detection_only_get_mut_mut(&mut self) -> Option<&mut ()> {
        self.option_detection_only_get_mut.as_mut()
    }
}
