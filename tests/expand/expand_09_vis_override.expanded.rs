#[fieldwork(vis = "pub(crate)", get, set)]
struct MyStruct {
    /// default visibility (should be `pub(crate)`)
    number: usize,
    /// this one overrides only the getter to be fully pub
    #[fieldwork(get(vis = "pub"))]
    enabled: bool,
    /// this one overrides only the getter to be private (no `pub`)
    #[fieldwork(get(vis = ""))]
    active: bool,
    #[fieldwork(vis = "pub")]
    other: (),
    #[fieldwork(vis = "pub", get(vis = ""))]
    double_override: (),
}
impl MyStruct {
    ///Returns a copy of default visibility (should be `pub(crate)`)
    pub(crate) fn number(&self) -> usize {
        self.number
    }
    ///Sets default visibility (should be `pub(crate)`), returning `&mut Self` for chaining
    pub(crate) fn set_number(&mut self, number: usize) -> &mut Self {
        self.number = number;
        self
    }
    ///Returns a copy of this one overrides only the getter to be fully pub
    pub fn enabled(&self) -> bool {
        self.enabled
    }
    ///Sets this one overrides only the getter to be fully pub, returning `&mut Self` for chaining
    pub(crate) fn set_enabled(&mut self, enabled: bool) -> &mut Self {
        self.enabled = enabled;
        self
    }
    ///Returns a copy of this one overrides only the getter to be private (no `pub`)
    fn active(&self) -> bool {
        self.active
    }
    ///Sets this one overrides only the getter to be private (no `pub`), returning `&mut Self` for chaining
    pub(crate) fn set_active(&mut self, active: bool) -> &mut Self {
        self.active = active;
        self
    }
    pub fn other(&self) -> &() {
        &self.other
    }
    pub fn set_other(&mut self, other: ()) -> &mut Self {
        self.other = other;
        self
    }
    fn double_override(&self) -> &() {
        &self.double_override
    }
    pub fn set_double_override(&mut self, double_override: ()) -> &mut Self {
        self.double_override = double_override;
        self
    }
}
#[fieldwork(get, set)]
struct MyStruct2 {
    /// default visibility
    number: usize,
    /// this one overrides only the getter to be fully pub
    #[fieldwork(get(vis = "pub(crate)"))]
    enabled: bool,
    /// this one overrides only the getter to be private (no `pub`)
    #[fieldwork(get(vis = ""))]
    active: bool,
    #[fieldwork(vis = "pub(crate)")]
    other: (),
    #[fieldwork(vis = "pub(crate)", get(vis = ""))]
    method_override_field: (),
}
impl MyStruct2 {
    ///Returns a copy of default visibility
    pub fn number(&self) -> usize {
        self.number
    }
    ///Sets default visibility, returning `&mut Self` for chaining
    pub fn set_number(&mut self, number: usize) -> &mut Self {
        self.number = number;
        self
    }
    ///Returns a copy of this one overrides only the getter to be fully pub
    pub(crate) fn enabled(&self) -> bool {
        self.enabled
    }
    ///Sets this one overrides only the getter to be fully pub, returning `&mut Self` for chaining
    pub fn set_enabled(&mut self, enabled: bool) -> &mut Self {
        self.enabled = enabled;
        self
    }
    ///Returns a copy of this one overrides only the getter to be private (no `pub`)
    fn active(&self) -> bool {
        self.active
    }
    ///Sets this one overrides only the getter to be private (no `pub`), returning `&mut Self` for chaining
    pub fn set_active(&mut self, active: bool) -> &mut Self {
        self.active = active;
        self
    }
    pub(crate) fn other(&self) -> &() {
        &self.other
    }
    pub(crate) fn set_other(&mut self, other: ()) -> &mut Self {
        self.other = other;
        self
    }
    fn method_override_field(&self) -> &() {
        &self.method_override_field
    }
    pub(crate) fn set_method_override_field(
        &mut self,
        method_override_field: (),
    ) -> &mut Self {
        self.method_override_field = method_override_field;
        self
    }
}
/// Enum: vis override at the enum level
#[fieldwork(vis = "pub(crate)", get, set)]
enum State {
    Active { value: i32 },
    Inactive { value: i32 },
}
impl State {
    pub(crate) fn value(&self) -> i32 {
        match self {
            Self::Active { value, .. } | Self::Inactive { value, .. } => *value,
        }
    }
    pub(crate) fn set_value(&mut self, value: i32) -> &mut Self {
        match self {
            Self::Active { value: value_binding, .. } => {
                *value_binding = value;
            }
            Self::Inactive { value: value_binding, .. } => {
                *value_binding = value;
            }
        }
        self
    }
}
/// Item-method-level vis: get(vis = ...) in the item attribute restricts all getters
#[fieldwork(get(vis = "pub(crate)"), set)]
struct MethodVis {
    value: i32,
    name: String,
}
impl MethodVis {
    pub(crate) fn value(&self) -> i32 {
        self.value
    }
    pub fn set_value(&mut self, value: i32) -> &mut Self {
        self.value = value;
        self
    }
    pub(crate) fn name(&self) -> &str {
        &*self.name
    }
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }
}
/// Enum: field-level vis override
#[fieldwork(vis = "pub(crate)", get, set)]
enum Mixed {
    Alpha {
        /// pub(crate) by default
        shared: i32,
        /// this field overrides to pub
        #[fieldwork(get(vis = "pub"))]
        prominent: String,
    },
    Beta { shared: i32, #[fieldwork(vis = "pub")] prominent: String },
}
impl Mixed {
    ///Borrows this field overrides to pub
    pub fn prominent(&self) -> &str {
        match self {
            Self::Alpha { prominent, .. } | Self::Beta { prominent, .. } => &**prominent,
        }
    }
    ///Sets this field overrides to pub, returning `&mut Self` for chaining
    pub(crate) fn set_prominent(&mut self, prominent: String) -> &mut Self {
        match self {
            Self::Alpha { prominent: prominent_binding, .. } => {
                *prominent_binding = prominent;
            }
            Self::Beta { prominent: prominent_binding, .. } => {
                *prominent_binding = prominent;
            }
        }
        self
    }
    ///Borrows pub(crate) by default
    pub(crate) fn shared(&self) -> i32 {
        match self {
            Self::Alpha { shared, .. } | Self::Beta { shared, .. } => *shared,
        }
    }
    ///Sets pub(crate) by default, returning `&mut Self` for chaining
    pub(crate) fn set_shared(&mut self, shared: i32) -> &mut Self {
        match self {
            Self::Alpha { shared: shared_binding, .. } => {
                *shared_binding = shared;
            }
            Self::Beta { shared: shared_binding, .. } => {
                *shared_binding = shared;
            }
        }
        self
    }
}
