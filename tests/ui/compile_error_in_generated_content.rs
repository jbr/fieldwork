#[derive(fieldwork::Fieldwork)]
struct Something {
    #[fieldwork(get, deref = "()")]
    string: String,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get)]
struct SomethingElse {
    #[fieldwork(deref = "()")]
    string: String,
}

fn main() {}
