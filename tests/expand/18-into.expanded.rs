#[fieldwork(get, set, get_mut, with)]
struct AcceptsAnythingInto {
    #[fieldwork(option_set_some, into)]
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
        self.string = Some(string.into());
        self
    }
    #[must_use]
    pub fn with_string(mut self, string: impl Into<String>) -> Self {
        self.string = Some(string.into());
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
