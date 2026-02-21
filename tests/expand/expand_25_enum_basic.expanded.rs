/// Full coverage: all variants have x and y; partial: button, delta
#[fieldwork(get, get_mut, set, with, without, take, into_field)]
enum Event {
    Click { x: i32, y: i32, button: u8 },
    Move { x: i32, y: i32 },
    Scroll { x: i32, y: i32, delta: f32 },
}
impl Event {
    pub fn button(&self) -> Option<u8> {
        match self {
            Self::Click { button, .. } => Some(*button),
            _ => None,
        }
    }
    pub fn button_mut(&mut self) -> Option<&mut u8> {
        match self {
            Self::Click { button, .. } => Some(button),
            _ => None,
        }
    }
    pub fn delta(&self) -> Option<f32> {
        match self {
            Self::Scroll { delta, .. } => Some(*delta),
            _ => None,
        }
    }
    pub fn delta_mut(&mut self) -> Option<&mut f32> {
        match self {
            Self::Scroll { delta, .. } => Some(delta),
            _ => None,
        }
    }
    pub fn x(&self) -> i32 {
        match self {
            Self::Click { x, .. } | Self::Move { x, .. } | Self::Scroll { x, .. } => *x,
        }
    }
    pub fn x_mut(&mut self) -> &mut i32 {
        match self {
            Self::Click { x, .. } => x,
            Self::Move { x, .. } => x,
            Self::Scroll { x, .. } => x,
        }
    }
    pub fn set_x(&mut self, x: i32) -> &mut Self {
        match self {
            Self::Click { x: x_binding, .. } => {
                *x_binding = x;
            }
            Self::Move { x: x_binding, .. } => {
                *x_binding = x;
            }
            Self::Scroll { x: x_binding, .. } => {
                *x_binding = x;
            }
        }
        self
    }
    #[must_use]
    pub fn with_x(mut self, x: i32) -> Self {
        match &mut self {
            Self::Click { x: x_binding, .. } => {
                *x_binding = x;
            }
            Self::Move { x: x_binding, .. } => {
                *x_binding = x;
            }
            Self::Scroll { x: x_binding, .. } => {
                *x_binding = x;
            }
        }
        self
    }
    pub fn y(&self) -> i32 {
        match self {
            Self::Click { y, .. } | Self::Move { y, .. } | Self::Scroll { y, .. } => *y,
        }
    }
    pub fn y_mut(&mut self) -> &mut i32 {
        match self {
            Self::Click { y, .. } => y,
            Self::Move { y, .. } => y,
            Self::Scroll { y, .. } => y,
        }
    }
    pub fn set_y(&mut self, y: i32) -> &mut Self {
        match self {
            Self::Click { y: y_binding, .. } => {
                *y_binding = y;
            }
            Self::Move { y: y_binding, .. } => {
                *y_binding = y;
            }
            Self::Scroll { y: y_binding, .. } => {
                *y_binding = y;
            }
        }
        self
    }
    #[must_use]
    pub fn with_y(mut self, y: i32) -> Self {
        match &mut self {
            Self::Click { y: y_binding, .. } => {
                *y_binding = y;
            }
            Self::Move { y: y_binding, .. } => {
                *y_binding = y;
            }
            Self::Scroll { y: y_binding, .. } => {
                *y_binding = y;
            }
        }
        self
    }
}
/// Newtype-style enum with a String field across all variants
#[fieldwork(get, set, into_field)]
enum Status {
    Active { name: String },
    Inactive { name: String },
}
impl Status {
    pub fn name(&self) -> &str {
        match self {
            Self::Active { name, .. } | Self::Inactive { name, .. } => &**name,
        }
    }
    pub fn set_name(&mut self, name: String) -> &mut Self {
        match self {
            Self::Active { name: name_binding, .. } => {
                *name_binding = name;
            }
            Self::Inactive { name: name_binding, .. } => {
                *name_binding = name;
            }
        }
        self
    }
    pub fn into_name(self) -> String {
        match self {
            Self::Active { name, .. } | Self::Inactive { name, .. } => name,
        }
    }
}
/// Option fields: take + without
#[fieldwork(get, take, without)]
enum Session {
    Authenticated { token: Option<String>, user_id: u32 },
    Anonymous { token: Option<String> },
}
impl Session {
    pub fn token(&self) -> Option<&str> {
        match self {
            Self::Authenticated { token, .. } | Self::Anonymous { token, .. } => {
                token.as_deref()
            }
        }
    }
    #[must_use]
    pub fn without_token(mut self) -> Self {
        match &mut self {
            Self::Authenticated { token, .. } => {
                *token = None;
            }
            Self::Anonymous { token, .. } => {
                *token = None;
            }
        }
        self
    }
    pub fn take_token(&mut self) -> Option<String> {
        match self {
            Self::Authenticated { token, .. } | Self::Anonymous { token, .. } => {
                token.take()
            }
        }
    }
    pub fn user_id(&self) -> Option<u32> {
        match self {
            Self::Authenticated { user_id, .. } => Some(*user_id),
            _ => None,
        }
    }
}
