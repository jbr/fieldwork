#[fieldwork(get, get_mut, const_fn)]
struct Counters {
    /// how many times this has happened
    count: usize,
    /// whether the thing is on
    enabled: bool,
}
impl Counters {
    ///Returns a copy of how many times this has happened
    pub const fn count(&self) -> usize {
        self.count
    }
    ///Mutably borrow how many times this has happened
    pub const fn count_mut(&mut self) -> &mut usize {
        &mut self.count
    }
    ///Returns a copy of whether the thing is on
    pub const fn enabled(&self) -> bool {
        self.enabled
    }
    ///Mutably borrow whether the thing is on
    pub const fn enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }
}
#[fieldwork(get)]
struct Mixed {
    /// borrowed by an ordinary getter
    name: String,
    /// read in a const context
    #[field(const_fn)]
    port: u16,
}
impl Mixed {
    ///Borrows borrowed by an ordinary getter
    pub fn name(&self) -> &str {
        &*self.name
    }
    ///Returns a copy of read in a const context
    pub const fn port(&self) -> u16 {
        self.port
    }
}
#[fieldwork(get, set, const_fn)]
struct Override {
    /// read in a const context
    port: u16,
    /// opted back out of const
    #[field(const_fn = false)]
    name: String,
}
impl Override {
    ///Returns a copy of read in a const context
    pub const fn port(&self) -> u16 {
        self.port
    }
    ///Sets read in a const context, returning `&mut Self` for chaining
    pub const fn set_port(&mut self, port: u16) -> &mut Self {
        self.port = port;
        self
    }
    ///Borrows opted back out of const
    pub fn name(&self) -> &str {
        &*self.name
    }
    ///Sets opted back out of const, returning `&mut Self` for chaining
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }
}
#[fieldwork(get, get_mut, const_fn, deref = false)]
struct NoDeref {
    /// borrowed without auto-deref
    bytes: [u8; 4],
    /// borrowed without auto-deref
    label: Option<String>,
}
impl NoDeref {
    ///Borrows borrowed without auto-deref
    pub const fn bytes(&self) -> &[u8; 4] {
        &self.bytes
    }
    ///Mutably borrow borrowed without auto-deref
    pub const fn bytes_mut(&mut self) -> &mut [u8; 4] {
        &mut self.bytes
    }
    ///Borrows borrowed without auto-deref
    pub const fn label(&self) -> Option<&String> {
        self.label.as_ref()
    }
    ///Mutably borrow borrowed without auto-deref
    pub const fn label_mut(&mut self) -> Option<&mut String> {
        self.label.as_mut()
    }
}
#[fieldwork(get(const_fn), set, with, without, take)]
struct MethodLevel {
    /// only the getter is const
    enabled: bool,
    /// only the getter is const
    retries: Option<u8>,
}
impl MethodLevel {
    ///Returns a copy of only the getter is const
    pub const fn enabled(&self) -> bool {
        self.enabled
    }
    ///Sets only the getter is const, returning `&mut Self` for chaining
    pub fn set_enabled(&mut self, enabled: bool) -> &mut Self {
        self.enabled = enabled;
        self
    }
    ///Owned chainable setter for only the getter is const, returning `Self`
    #[must_use]
    pub fn with_enabled(mut self) -> Self {
        self.enabled = true;
        self
    }
    ///Owned chainable setter for only the getter is const, returning `Self`
    #[must_use]
    pub fn without_enabled(mut self) -> Self {
        self.enabled = false;
        self
    }
    ///Returns a copy of only the getter is const
    pub const fn retries(&self) -> Option<u8> {
        self.retries
    }
    ///Sets only the getter is const, returning `&mut Self` for chaining
    pub fn set_retries(&mut self, retries: Option<u8>) -> &mut Self {
        self.retries = retries;
        self
    }
    ///Owned chainable setter for only the getter is const, returning `Self`
    #[must_use]
    pub fn with_retries(mut self, retries: u8) -> Self {
        self.retries = Some(retries);
        self
    }
    ///Owned chainable setter for only the getter is const, returning `Self`
    #[must_use]
    pub fn without_retries(mut self) -> Self {
        self.retries = None;
        self
    }
    ///Takes only the getter is const, leaving a None in its place
    pub fn take_retries(&mut self) -> Option<u8> {
        self.retries.take()
    }
}
