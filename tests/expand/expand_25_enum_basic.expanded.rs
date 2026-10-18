/// Full coverage: all variants have x and y; partial: button, delta
#[fieldwork(get, get_mut, set, with, without, take, into_field)]
enum Event {
    Click { x: i32, y: i32, button: u8 },
    Move { x: i32, y: i32 },
    Scroll { x: i32, y: i32, delta: f32 },
}
impl Event {
    #[inline]
    pub fn button(&self) -> Option<u8> {
        match self {
            Self::Click { button, .. } => Some(*button),
            _ => None,
        }
    }
    #[inline]
    pub fn button_mut(&mut self) -> Option<&mut u8> {
        match self {
            Self::Click { button, .. } => Some(button),
            _ => None,
        }
    }
    #[inline]
    pub fn delta(&self) -> Option<f32> {
        match self {
            Self::Scroll { delta, .. } => Some(*delta),
            _ => None,
        }
    }
    #[inline]
    pub fn delta_mut(&mut self) -> Option<&mut f32> {
        match self {
            Self::Scroll { delta, .. } => Some(delta),
            _ => None,
        }
    }
    #[inline]
    pub fn x(&self) -> i32 {
        match self {
            Self::Click { x, .. } | Self::Move { x, .. } | Self::Scroll { x, .. } => *x,
        }
    }
    #[inline]
    pub fn x_mut(&mut self) -> &mut i32 {
        match self {
            Self::Click { x, .. } => x,
            Self::Move { x, .. } => x,
            Self::Scroll { x, .. } => x,
        }
    }
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn y(&self) -> i32 {
        match self {
            Self::Click { y, .. } | Self::Move { y, .. } | Self::Scroll { y, .. } => *y,
        }
    }
    #[inline]
    pub fn y_mut(&mut self) -> &mut i32 {
        match self {
            Self::Click { y, .. } => y,
            Self::Move { y, .. } => y,
            Self::Scroll { y, .. } => y,
        }
    }
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn name(&self) -> &str {
        match self {
            Self::Active { name, .. } | Self::Inactive { name, .. } => &**name,
        }
    }
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn token(&self) -> Option<&str> {
        match self {
            Self::Authenticated { token, .. } | Self::Anonymous { token, .. } => {
                token.as_deref()
            }
        }
    }
    #[inline]
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
    #[inline]
    pub fn take_token(&mut self) -> Option<String> {
        match self {
            Self::Authenticated { token, .. } | Self::Anonymous { token, .. } => {
                token.take()
            }
        }
    }
    #[inline]
    pub fn user_id(&self) -> Option<u32> {
        match self {
            Self::Authenticated { user_id, .. } => Some(*user_id),
            _ => None,
        }
    }
}
