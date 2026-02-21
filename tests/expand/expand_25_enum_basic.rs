/// Full coverage: all variants have x and y; partial: button, delta
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, get_mut, set, with, without, take, into_field)]
enum Event {
    Click { x: i32, y: i32, button: u8 },
    Move { x: i32, y: i32 },
    Scroll { x: i32, y: i32, delta: f32 },
}

/// Newtype-style enum with a String field across all variants
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set, into_field)]
enum Status {
    Active { name: String },
    Inactive { name: String },
}

/// Option fields: take + without
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, take, without)]
enum Session {
    Authenticated { token: Option<String>, user_id: u32 },
    Anonymous { token: Option<String> },
}
