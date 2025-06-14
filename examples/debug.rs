#![allow(dead_code)]

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get(copy = false))]
struct Collection {
    /// length
    len: usize,

    /// enabled
    enabled: bool,
}

fn main() {}
