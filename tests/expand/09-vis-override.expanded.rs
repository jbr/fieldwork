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
    ///Borrows default visibility (should be `pub(crate)`)
    pub(crate) fn number(&self) -> &usize {
        &self.number
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
    ///Borrows default visibility
    pub fn number(&self) -> &usize {
        &self.number
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
