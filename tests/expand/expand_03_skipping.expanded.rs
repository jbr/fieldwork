#[fieldwork(get, set, with, get_mut)]
struct MyStruct<T> {
    /// this number is cool
    number: usize,
    /// is this struct on or not
    #[fieldwork(skip)]
    enabled: bool,
    /// it's really whatever you want
    #[fieldwork(skip = true)]
    generic: T,
}
impl<T> MyStruct<T> {
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
}
#[fieldwork(get, set, with, get_mut)]
struct AnotherInterface {
    number: usize,
    #[fieldwork = false]
    enabled: bool,
}
impl AnotherInterface {
    pub fn number(&self) -> usize {
        self.number
    }
    pub fn number_mut(&mut self) -> &mut usize {
        &mut self.number
    }
    pub fn set_number(&mut self, number: usize) -> &mut Self {
        self.number = number;
        self
    }
    #[must_use]
    pub fn with_number(mut self, number: usize) -> Self {
        self.number = number;
        self
    }
}
#[fieldwork(set, get)]
struct SetAndGet<T> {
    /// this number is cool
    number: usize,
    /// is this struct on or not
    #[fieldwork(get(skip = true))]
    enabled: bool,
    /// it's really whatever you want
    #[fieldwork(set(skip))]
    generic: T,
}
impl<T> SetAndGet<T> {
    ///Returns a copy of this number is cool
    pub fn number(&self) -> usize {
        self.number
    }
    ///Sets this number is cool, returning `&mut Self` for chaining
    pub fn set_number(&mut self, number: usize) -> &mut Self {
        self.number = number;
        self
    }
    ///Sets is this struct on or not, returning `&mut Self` for chaining
    pub fn set_enabled(&mut self, enabled: bool) -> &mut Self {
        self.enabled = enabled;
        self
    }
    ///Borrows it's really whatever you want
    pub fn generic(&self) -> &T {
        &self.generic
    }
}
#[fieldwork(get, set, with, get_mut)]
struct SkipWithAssignment<T> {
    #[fieldwork(with = false)]
    no_with: bool,
    #[fieldwork(get = false)]
    no_get: T,
}
impl<T> SkipWithAssignment<T> {
    pub fn no_with(&self) -> bool {
        self.no_with
    }
    pub fn no_with_mut(&mut self) -> &mut bool {
        &mut self.no_with
    }
    pub fn set_no_with(&mut self, no_with: bool) -> &mut Self {
        self.no_with = no_with;
        self
    }
    pub fn no_get_mut(&mut self) -> &mut T {
        &mut self.no_get
    }
    pub fn set_no_get(&mut self, no_get: T) -> &mut Self {
        self.no_get = no_get;
        self
    }
    #[must_use]
    pub fn with_no_get(mut self, no_get: T) -> Self {
        self.no_get = no_get;
        self
    }
}
#[fieldwork(get, get_mut = false)]
struct GetMutEqualsFalseDoesNothing<T> {
    field: T,
}
impl<T> GetMutEqualsFalseDoesNothing<T> {
    pub fn field(&self) -> &T {
        &self.field
    }
}
/// Enum: skip a specific field; it's excluded from all method generation
#[fieldwork(get, set)]
enum SkipField {
    Alpha { name: String, #[field(skip)] internal: u32 },
    Beta { name: String },
}
impl SkipField {
    pub fn name(&self) -> &str {
        match self {
            Self::Alpha { name, .. } | Self::Beta { name, .. } => &**name,
        }
    }
    pub fn set_name(&mut self, name: String) -> &mut Self {
        match self {
            Self::Alpha { name: name_binding, .. } => {
                *name_binding = name;
            }
            Self::Beta { name: name_binding, .. } => {
                *name_binding = name;
            }
        }
        self
    }
}
/// Enum: #[field = false] on any occurrence globally vetoes the entire virtual field.
/// Although Active and Inactive both have `value: i32`, Debug's veto suppresses all
/// generated methods for `value`. `extra` only exists in Debug and is also vetoed.
/// Use this to suppress accessor generation for a field that is unsuitable in one variant
/// (e.g. internal debug state that shouldn't be part of the public interface).
#[fieldwork(get, set)]
enum SkipVariant {
    Active { value: i32 },
    Debug { #[field = false] value: i32, #[field = false] extra: String },
    Inactive { value: i32 },
}
impl SkipVariant {}
/// Enum: per-method field skip
#[fieldwork(get, set)]
enum PerMethodSkip {
    Foo {
        #[fieldwork(get(skip))]
        write_only: i32,
        #[fieldwork(set(skip))]
        read_only: i32,
    },
    Bar { write_only: i32, read_only: i32 },
}
impl PerMethodSkip {
    pub fn read_only(&self) -> i32 {
        match self {
            Self::Foo { read_only, .. } | Self::Bar { read_only, .. } => *read_only,
        }
    }
    pub fn set_write_only(&mut self, write_only: i32) -> &mut Self {
        match self {
            Self::Foo { write_only: write_only_binding, .. } => {
                *write_only_binding = write_only;
            }
            Self::Bar { write_only: write_only_binding, .. } => {
                *write_only_binding = write_only;
            }
        }
        self
    }
}
