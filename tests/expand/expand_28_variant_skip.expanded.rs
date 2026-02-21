/// `#[variant(skip)]` excludes a variant: only active variants contribute fields
#[fieldwork(get, set)]
enum Message {
    Text { content: String },
    Image { url: String },
    #[variant(skip)]
    Hidden { data: String },
}
impl Message {
    pub fn content(&self) -> Option<&str> {
        match self {
            Self::Text { content, .. } => Some(&**content),
            _ => None,
        }
    }
    pub fn url(&self) -> Option<&str> {
        match self {
            Self::Image { url, .. } => Some(&**url),
            _ => None,
        }
    }
}
/// `#[variant = false]` is an alternative skip syntax
#[fieldwork(get)]
enum Status {
    Active { value: i32 },
    #[variant = false]
    Deprecated { value: i32 },
    Inactive { value: i32 },
}
impl Status {
    pub fn value(&self) -> Option<i32> {
        match self {
            Self::Active { value, .. } => Some(*value),
            Self::Inactive { value, .. } => Some(*value),
            _ => None,
        }
    }
}
/// `#[variant(skip = true)]` explicitly skips a variant
#[fieldwork(get)]
enum Command {
    Write { data: String },
    #[variant(skip = true)]
    Noop {},
    Read { data: String },
}
impl Command {
    pub fn data(&self) -> Option<&str> {
        match self {
            Self::Write { data, .. } => Some(&**data),
            Self::Read { data, .. } => Some(&**data),
            _ => None,
        }
    }
}
/// `#[variant(skip = false)]` explicitly opts in (same as no attribute)
#[fieldwork(get)]
enum Priority {
    High { level: u8 },
    #[variant(skip = false)]
    Low { level: u8 },
}
impl Priority {
    pub fn level(&self) -> u8 {
        match self {
            Self::High { level, .. } | Self::Low { level, .. } => *level,
        }
    }
}
/// bare `#[variant]` is a no-op
#[fieldwork(get)]
enum Mode {
    #[variant]
    Normal { speed: u32 },
    Fast { speed: u32 },
}
impl Mode {
    pub fn speed(&self) -> u32 {
        match self {
            Self::Normal { speed, .. } | Self::Fast { speed, .. } => *speed,
        }
    }
}
