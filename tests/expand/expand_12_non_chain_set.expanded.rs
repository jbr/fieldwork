#[fieldwork(set)]
struct MyStruct<T> {
    number: usize,
    /// opting out
    #[fieldwork(set(chain = false))]
    enabled: bool,
    generic: T,
}
impl<T> MyStruct<T> {
    pub fn set_number(&mut self, number: usize) -> &mut Self {
        self.number = number;
        self
    }
    ///Sets opting out
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    pub fn set_generic(&mut self, generic: T) -> &mut Self {
        self.generic = generic;
        self
    }
}
#[fieldwork(set(chain = false))]
struct MyStruct2<T> {
    /// opted out at struct-method level
    number: usize,
    #[fieldwork(set(chain = true))]
    /// opting back in
    enabled: bool,
    #[fieldwork(set(chain))]
    /// opting back in
    generic: T,
}
impl<T> MyStruct2<T> {
    ///Sets opted out at struct-method level
    pub fn set_number(&mut self, number: usize) {
        self.number = number;
    }
    ///Sets opting back in, returning `&mut Self` for chaining
    pub fn set_enabled(&mut self, enabled: bool) -> &mut Self {
        self.enabled = enabled;
        self
    }
    ///Sets opting back in, returning `&mut Self` for chaining
    pub fn set_generic(&mut self, generic: T) -> &mut Self {
        self.generic = generic;
        self
    }
}
/// Enum: chain = false at enum-method level; setter returns ()
#[fieldwork(set(chain = false))]
enum Point {
    TwoD { x: i32, y: i32 },
    ThreeD { x: i32, y: i32, z: i32 },
}
impl Point {
    pub fn set_x(&mut self, x: i32) {
        match self {
            Self::TwoD { x: x_binding, .. } => {
                *x_binding = x;
            }
            Self::ThreeD { x: x_binding, .. } => {
                *x_binding = x;
            }
        }
    }
    pub fn set_y(&mut self, y: i32) {
        match self {
            Self::TwoD { y: y_binding, .. } => {
                *y_binding = y;
            }
            Self::ThreeD { y: y_binding, .. } => {
                *y_binding = y;
            }
        }
    }
}
/// Enum: chain = false for specific field, opting back in for another
#[fieldwork(set)]
enum Mixed {
    A { #[fieldwork(set(chain = false))] no_chain: i32, chained: i32 },
    B { no_chain: i32, chained: i32 },
}
impl Mixed {
    pub fn set_chained(&mut self, chained: i32) -> &mut Self {
        match self {
            Self::A { chained: chained_binding, .. } => {
                *chained_binding = chained;
            }
            Self::B { chained: chained_binding, .. } => {
                *chained_binding = chained;
            }
        }
        self
    }
    pub fn set_no_chain(&mut self, no_chain: i32) {
        match self {
            Self::A { no_chain: no_chain_binding, .. } => {
                *no_chain_binding = no_chain;
            }
            Self::B { no_chain: no_chain_binding, .. } => {
                *no_chain_binding = no_chain;
            }
        }
    }
}
