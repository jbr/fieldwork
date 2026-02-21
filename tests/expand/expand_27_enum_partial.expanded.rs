/// partial = "result" for set: partial-coverage fields return Result<(), T>
/// Full-coverage fields (x, y) are unaffected
#[fieldwork(set(partial = "result"))]
enum Event {
    Click { x: i32, y: i32, button: u8 },
    Move { x: i32, y: i32 },
}
/// partial = "noop" for set: silently does nothing on wrong variant
#[fieldwork(set(partial = "noop"))]
enum Notification {
    Alert { message: String, priority: u8 },
    Info { message: String },
}
/// partial = "panic" for set: panics on wrong variant
#[fieldwork(set(partial = "panic"))]
enum Request {
    Get { path: String },
    Post { path: String, body: String },
}
/// partial = "result" for into_field: returns Result<T, Self>
/// Full-coverage fields return T directly
#[fieldwork(into_field(partial = "result"))]
enum Response {
    Success { data: String, code: u16 },
    Error { code: u16 },
}
/// partial = "panic" for into_field: panics on wrong variant
#[fieldwork(into_field(partial = "panic"))]
enum Outcome {
    Ok { value: String },
    Err { reason: String },
}
/// with(partial = "noop"): returns Self, silently no-ops on wrong variant
#[fieldwork(with(partial = "noop"))]
enum Builder {
    Full { x: i32, y: i32, extra: String },
    Minimal { x: i32, y: i32 },
}
/// with(partial = "panic"): returns Self, panics on wrong variant
#[fieldwork(with(partial = "panic"))]
enum Strict {
    Complete { name: String, value: u32 },
    Simple { name: String },
}
/// Mix: get (auto partial/full) + set(partial = "result") together
#[fieldwork(get, set(partial = "result"))]
enum Mixed {
    A { shared: i32, a_only: String },
    B { shared: i32, b_only: u8 },
}
