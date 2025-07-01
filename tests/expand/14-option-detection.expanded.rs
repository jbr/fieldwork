#[fieldwork(get, get_mut)]
struct OptionBehavior {
    #[fieldwork(deref = "Option<&str>")]
    option_deref: Option<String>,
    #[fieldwork(deref = str)]
    option_deref_other_style: Option<String>,
    #[fieldwork(option_borrow_inner = false)]
    no_option_detection: Option<String>,
    option_detection: Option<()>,
    #[fieldwork(get_mut(option_borrow_inner = false))]
    nothing_fancy_for_get_mut: Option<()>,
}
impl OptionBehavior {
    pub fn option_deref(&self) -> Option<&str> {
        self.option_deref.as_deref()
    }
    pub fn option_deref_mut(&mut self) -> Option<&mut str> {
        self.option_deref.as_deref_mut()
    }
    pub fn option_deref_other_style(&self) -> &str {
        &*self.option_deref_other_style
    }
    pub fn option_deref_other_style_mut(&mut self) -> &mut str {
        &mut *self.option_deref_other_style
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
#[fieldwork(get, get_mut, option_borrow_inner = false)]
struct OptInOption {
    #[fieldwork(deref = "Option<&str>", option_borrow_inner = true)]
    option_deref: Option<String>,
    #[fieldwork(deref = str, option_borrow_inner = true)]
    option_deref_other_style: Option<String>,
    #[fieldwork(option_borrow_inner)]
    option_detection: Option<String>,
    no_option_detection: Option<()>,
    #[fieldwork(get_mut(option_borrow_inner = true))]
    option_detection_only_get_mut: Option<()>,
}
impl OptInOption {
    pub fn option_deref(&self) -> Option<&str> {
        self.option_deref.as_deref()
    }
    pub fn option_deref_mut(&mut self) -> Option<&mut str> {
        self.option_deref.as_deref_mut()
    }
    pub fn option_deref_other_style(&self) -> &str {
        &*self.option_deref_other_style
    }
    pub fn option_deref_other_style_mut(&mut self) -> &mut str {
        &mut *self.option_deref_other_style
    }
    pub fn option_detection(&self) -> Option<&str> {
        self.option_detection.as_deref()
    }
    pub fn option_detection_mut(&mut self) -> Option<&mut str> {
        self.option_detection.as_deref_mut()
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
#[fieldwork(get(option_borrow_inner = true), get_mut, option_borrow_inner = false)]
struct OptionOnlyForGet {
    #[fieldwork(get(option_borrow_inner = false))]
    no_option_detection: Option<()>,
    #[fieldwork(get(deref = str))]
    option_deref: Option<String>,
    #[fieldwork(option_borrow_inner = true)]
    field_overrides: Option<String>,
    option_only_for_get: Option<()>,
    #[fieldwork(get_mut(option_borrow_inner), get(option_borrow_inner = false))]
    option_detection_only_get_mut: Option<()>,
}
impl OptionOnlyForGet {
    pub fn no_option_detection(&self) -> &Option<()> {
        &self.no_option_detection
    }
    pub fn no_option_detection_mut(&mut self) -> &mut Option<()> {
        &mut self.no_option_detection
    }
    pub fn option_deref(&self) -> &str {
        &*self.option_deref
    }
    pub fn option_deref_mut(&mut self) -> &mut Option<String> {
        &mut self.option_deref
    }
    pub fn field_overrides(&self) -> Option<&str> {
        self.field_overrides.as_deref()
    }
    pub fn field_overrides_mut(&mut self) -> Option<&mut str> {
        self.field_overrides.as_deref_mut()
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
#[fieldwork(get, get_mut, option_borrow_inner = false, deref = false)]
struct OptionAndDerefInteraction {
    a: Option<String>,
    #[fieldwork(option_borrow_inner)]
    b: Option<String>,
    #[fieldwork(option_borrow_inner, deref)]
    c: Option<String>,
    #[fieldwork(deref)]
    d: Option<String>,
    #[fieldwork(option_borrow_inner, deref = "Option<&CustomDeref>")]
    e: Option<CustomOwned>,
}
impl OptionAndDerefInteraction {
    pub fn a(&self) -> &Option<String> {
        &self.a
    }
    pub fn a_mut(&mut self) -> &mut Option<String> {
        &mut self.a
    }
    pub fn b(&self) -> Option<&String> {
        self.b.as_ref()
    }
    pub fn b_mut(&mut self) -> Option<&mut String> {
        self.b.as_mut()
    }
    pub fn c(&self) -> Option<&str> {
        self.c.as_deref()
    }
    pub fn c_mut(&mut self) -> Option<&mut str> {
        self.c.as_deref_mut()
    }
    pub fn d(&self) -> &Option<String> {
        &self.d
    }
    pub fn d_mut(&mut self) -> &mut Option<String> {
        &mut self.d
    }
    pub fn e(&self) -> Option<&CustomDeref> {
        self.e.as_deref()
    }
    pub fn e_mut(&mut self) -> Option<&mut CustomDeref> {
        self.e.as_deref_mut()
    }
}
