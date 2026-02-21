// #[variant(unknown)] — unrecognized bare option
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get)]
enum A {
    #[variant(unknown)]
    Foo { x: i32 },
    Bar { x: i32 },
}

// #[variant(skip = "not_a_bool")] — non-bool value for skip
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get)]
enum B {
    #[variant(skip = "not_a_bool")]
    Foo { x: i32 },
    Bar { x: i32 },
}

// #[variant(unknown = true)] — unrecognized key in assignment
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get)]
enum C {
    #[variant(unknown = true)]
    Foo { x: i32 },
    Bar { x: i32 },
}

// #[variant = "string"] — string literal instead of bool
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get)]
enum D {
    #[variant = "string"]
    Foo { x: i32 },
    Bar { x: i32 },
}

// #[variant(42 = true)] — non-path lhs in assignment
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get)]
enum E {
    #[variant(42 = true)]
    Foo { x: i32 },
    Bar { x: i32 },
}

// #[variant(1.0)] — unrecognized expression type in list
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get)]
enum F {
    #[variant(1.0)]
    Foo { x: i32 },
    Bar { x: i32 },
}

fn main() {}
