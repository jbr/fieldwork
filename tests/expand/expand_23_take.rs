#[derive(fieldwork::Fieldwork)]
struct MyStruct {
    #[field(take)]
    string: Option<String>,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(take)]
struct MyStruct2 {
    /// the users's name
    name: Option<String>,
    #[field(take = false)]
    not_take: Option<()>,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(take(template = "remove_{}"))]
struct MyStruct3 {
    /// the user's name
    name: Option<String>,
    ignored: bool,
}

/// Enum: take on full-coverage Option fields
#[derive(fieldwork::Fieldwork)]
#[fieldwork(take)]
enum WithTake {
    Cached {
        token: Option<String>,
        data: Option<Vec<u8>>,
    },
    Empty {
        token: Option<String>,
    },
}

/// Enum: take = false to opt out of specific field
#[derive(fieldwork::Fieldwork)]
#[fieldwork(take)]
enum SelectiveTake {
    A {
        include: Option<String>,
        #[field(take = false)]
        exclude: Option<u32>,
    },
    B {
        include: Option<String>,
        exclude: Option<u32>,
    },
}
