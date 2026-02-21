#[derive(fieldwork::Fieldwork)]
#[fieldwork(set)]
struct MyStruct<T> {
    number: usize,

    /// opting out
    #[fieldwork(set(chain = false))]
    enabled: bool,

    generic: T,
}
#[derive(fieldwork::Fieldwork)]
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

/// Enum: chain = false at enum-method level; setter returns ()
#[derive(fieldwork::Fieldwork)]
#[fieldwork(set(chain = false))]
enum Point {
    TwoD { x: i32, y: i32 },
    ThreeD { x: i32, y: i32, z: i32 },
}

/// Enum: chain = false for specific field, opting back in for another
#[derive(fieldwork::Fieldwork)]
#[fieldwork(set)]
enum Mixed {
    A {
        #[fieldwork(set(chain = false))]
        no_chain: i32,
        chained: i32,
    },
    B {
        no_chain: i32,
        chained: i32,
    },
}
