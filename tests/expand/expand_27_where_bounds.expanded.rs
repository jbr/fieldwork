/// Struct with a where clause (the basic case from issue #105)
#[fieldwork(get, set, with)]
struct MyStruct<T>
where
    T: Clone,
{
    field: T,
}
impl<T> MyStruct<T>
where
    T: Clone,
{
    #[inline]
    pub fn field(&self) -> &T {
        &self.field
    }
    #[inline]
    pub fn set_field(&mut self, field: T) -> &mut Self {
        self.field = field;
        self
    }
    #[inline]
    #[must_use]
    pub fn with_field(mut self, field: T) -> Self {
        self.field = field;
        self
    }
}
/// Enum with a where clause
#[fieldwork(get, set)]
enum Container<T>
where
    T: Clone,
{
    Filled { value: T, label: String },
    Empty { label: String },
}
impl<T> Container<T>
where
    T: Clone,
{
    #[inline]
    pub fn label(&self) -> &str {
        match self {
            Self::Filled { label, .. } | Self::Empty { label, .. } => &**label,
        }
    }
    #[inline]
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
    #[inline]
    pub fn value(&self) -> Option<&T> {
        match self {
            Self::Filled { value, .. } => Some(value),
            _ => None,
        }
    }
}
/// Struct with both inline bounds and a where clause
#[fieldwork(get, with)]
struct Mixed<T: std::fmt::Debug, U>
where
    U: Clone,
{
    debug_field: T,
    clone_field: U,
}
impl<T: std::fmt::Debug, U> Mixed<T, U>
where
    U: Clone,
{
    #[inline]
    pub fn debug_field(&self) -> &T {
        &self.debug_field
    }
    #[inline]
    #[must_use]
    pub fn with_debug_field(mut self, debug_field: T) -> Self {
        self.debug_field = debug_field;
        self
    }
    #[inline]
    pub fn clone_field(&self) -> &U {
        &self.clone_field
    }
    #[inline]
    #[must_use]
    pub fn with_clone_field(mut self, clone_field: U) -> Self {
        self.clone_field = clone_field;
        self
    }
}
/// Struct with both #[fieldwork(bounds = "...")] and a struct-level where clause — predicates merged
#[fieldwork(get, set, bounds = "U: std::fmt::Display")]
struct WithExplicitBounds<T, U>
where
    T: Clone,
{
    cloneable: T,
    displayable: U,
}
impl<T, U> WithExplicitBounds<T, U>
where
    T: Clone,
    U: std::fmt::Display,
{
    #[inline]
    pub fn cloneable(&self) -> &T {
        &self.cloneable
    }
    #[inline]
    pub fn set_cloneable(&mut self, cloneable: T) -> &mut Self {
        self.cloneable = cloneable;
        self
    }
    #[inline]
    pub fn displayable(&self) -> &U {
        &self.displayable
    }
    #[inline]
    pub fn set_displayable(&mut self, displayable: U) -> &mut Self {
        self.displayable = displayable;
        self
    }
}
