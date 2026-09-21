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
