struct MyStruct {
    #[field(take)]
    string: Option<String>,
}
impl MyStruct {
    pub fn take_string(&mut self) -> Option<String> {
        self.string.take()
    }
}
#[fieldwork(take)]
struct MyStruct2 {
    /// the users's name
    name: Option<String>,
    #[field(take = false)]
    not_take: Option<()>,
}
impl MyStruct2 {
    ///Takes the users's name, leaving a None in its place
    pub fn take_name(&mut self) -> Option<String> {
        self.name.take()
    }
}
#[fieldwork(take(template = "remove_{}"))]
struct MyStruct3 {
    /// the user's name
    name: Option<String>,
    ignored: bool,
}
impl MyStruct3 {
    ///Takes the user's name, leaving a None in its place
    pub fn remove_name(&mut self) -> Option<String> {
        self.name.take()
    }
}
/// Enum: take on full-coverage Option fields
#[fieldwork(take)]
enum WithTake {
    Cached { token: Option<String>, data: Option<Vec<u8>> },
    Empty { token: Option<String> },
}
impl WithTake {
    pub fn take_data(&mut self) -> Option<Vec<u8>> {
        match self {
            Self::Cached { data, .. } => data.take(),
            _ => None,
        }
    }
    pub fn take_token(&mut self) -> Option<String> {
        match self {
            Self::Cached { token, .. } | Self::Empty { token, .. } => token.take(),
        }
    }
}
/// Enum: take = false to opt out of specific field
#[fieldwork(take)]
enum SelectiveTake {
    A { include: Option<String>, #[field(take = false)] exclude: Option<u32> },
    B { include: Option<String>, exclude: Option<u32> },
}
impl SelectiveTake {
    pub fn take_include(&mut self) -> Option<String> {
        match self {
            Self::A { include, .. } | Self::B { include, .. } => include.take(),
        }
    }
}
