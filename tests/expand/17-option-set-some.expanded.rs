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
    pub fn no_option_detection(&self) -> Option<&bool> {
        self.no_option_detection.as_ref()
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
