#[fieldwork(get(template = "get_{}"), set(template = "assign_{}"), with, get_mut)]
struct MyStruct<T> {
    /// this number is cool
    #[fieldwork(rename = number_in_seconds)]
    number: usize,
    /// is this struct on or not
    enabled: bool,
    /// it's really whatever you want
    generic: T,
}
impl<T> MyStruct<T> {
    ///Returns a copy of this number is cool
    pub fn get_number_in_seconds(&self) -> usize {
        self.number
    }
    ///Mutably borrow this number is cool
    pub fn number_in_seconds_mut(&mut self) -> &mut usize {
        &mut self.number
    }
    ///Sets this number is cool, returning `&mut Self` for chaining
    pub fn assign_number_in_seconds(&mut self, number_in_seconds: usize) -> &mut Self {
        self.number = number_in_seconds;
        self
    }
    ///Owned chainable setter for this number is cool, returning `Self`
    #[must_use]
    pub fn with_number_in_seconds(mut self, number_in_seconds: usize) -> Self {
        self.number = number_in_seconds;
        self
    }
    ///Returns a copy of is this struct on or not
    pub fn get_enabled(&self) -> bool {
        self.enabled
    }
    ///Mutably borrow is this struct on or not
    pub fn enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }
    ///Sets is this struct on or not, returning `&mut Self` for chaining
    pub fn assign_enabled(&mut self, enabled: bool) -> &mut Self {
        self.enabled = enabled;
        self
    }
    ///Owned chainable setter for is this struct on or not, returning `Self`
    #[must_use]
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
    ///Borrows it's really whatever you want
    pub fn get_generic(&self) -> &T {
        &self.generic
    }
    ///Mutably borrow it's really whatever you want
    pub fn generic_mut(&mut self) -> &mut T {
        &mut self.generic
    }
    ///Sets it's really whatever you want, returning `&mut Self` for chaining
    pub fn assign_generic(&mut self, generic: T) -> &mut Self {
        self.generic = generic;
        self
    }
    ///Owned chainable setter for it's really whatever you want, returning `Self`
    #[must_use]
    pub fn with_generic(mut self, generic: T) -> Self {
        self.generic = generic;
        self
    }
}
/// Enum: custom method templates on get/set
#[fieldwork(get(template = "fetch_{}"), set(template = "assign_{}"))]
enum Config {
    Dev { host: String, port: u16 },
    Prod { host: String, port: u16 },
}
impl Config {
    pub fn fetch_host(&self) -> &str {
        match self {
            Self::Dev { host, .. } | Self::Prod { host, .. } => &**host,
        }
    }
    pub fn assign_host(&mut self, host: String) -> &mut Self {
        match self {
            Self::Dev { host: host_binding, .. } => {
                *host_binding = host;
            }
            Self::Prod { host: host_binding, .. } => {
                *host_binding = host;
            }
        }
        self
    }
    pub fn fetch_port(&self) -> u16 {
        match self {
            Self::Dev { port, .. } | Self::Prod { port, .. } => *port,
        }
    }
    pub fn assign_port(&mut self, port: u16) -> &mut Self {
        match self {
            Self::Dev { port: port_binding, .. } => {
                *port_binding = port;
            }
            Self::Prod { port: port_binding, .. } => {
                *port_binding = port;
            }
        }
        self
    }
}
