#[fieldwork(bounds = "T: Clone", get, set, with, get_mut)]
struct MyStruct<T> {
    /// this number is cool
    number: usize,
    /// is this struct on or not
    enabled: bool,
    /// must be clone
    generic: T,
}
impl<T> MyStruct<T>
where
    T: Clone,
{
    ///Returns a copy of this number is cool
    pub fn number(&self) -> usize {
        self.number
    }
    ///Mutably borrow this number is cool
    pub fn number_mut(&mut self) -> &mut usize {
        &mut self.number
    }
    ///Sets this number is cool, returning `&mut Self` for chaining
    pub fn set_number(&mut self, number: usize) -> &mut Self {
        self.number = number;
        self
    }
    ///Owned chainable setter for this number is cool, returning `Self`
    #[must_use]
    pub fn with_number(mut self, number: usize) -> Self {
        self.number = number;
        self
    }
    ///Returns a copy of is this struct on or not
    pub fn enabled(&self) -> bool {
        self.enabled
    }
    ///Mutably borrow is this struct on or not
    pub fn enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }
    ///Sets is this struct on or not, returning `&mut Self` for chaining
    pub fn set_enabled(&mut self, enabled: bool) -> &mut Self {
        self.enabled = enabled;
        self
    }
    ///Owned chainable setter for is this struct on or not, returning `Self`
    #[must_use]
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
    ///Borrows must be clone
    pub fn generic(&self) -> &T {
        &self.generic
    }
    ///Mutably borrow must be clone
    pub fn generic_mut(&mut self) -> &mut T {
        &mut self.generic
    }
    ///Sets must be clone, returning `&mut Self` for chaining
    pub fn set_generic(&mut self, generic: T) -> &mut Self {
        self.generic = generic;
        self
    }
    ///Owned chainable setter for must be clone, returning `Self`
    #[must_use]
    pub fn with_generic(mut self, generic: T) -> Self {
        self.generic = generic;
        self
    }
}
/// Enum: generic with explicit bounds
#[fieldwork(bounds = "T: Clone", get, set)]
enum Container<T> {
    Filled { value: T, label: String },
    Empty { label: String },
}
impl<T> Container<T>
where
    T: Clone,
{
    pub fn label(&self) -> &str {
        match self {
            Self::Filled { label, .. } | Self::Empty { label, .. } => &**label,
        }
    }
    pub fn set_label(&mut self, label: String) -> &mut Self {
        match self {
            Self::Filled { label: label_binding, .. } => {
                *label_binding = label;
            }
            Self::Empty { label: label_binding, .. } => {
                *label_binding = label;
            }
        }
        self
    }
    pub fn value(&self) -> Option<&T> {
        match self {
            Self::Filled { value, .. } => Some(value),
            _ => None,
        }
    }
}
