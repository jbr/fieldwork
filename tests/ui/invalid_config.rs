#[derive(fieldwork::Fieldwork)]
#[fieldwork(invalid_config)]
struct A {
    field: String,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(invalid_config = true)]
struct B {
    field: String,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set)]
struct C(#[fieldwork(invalid)] String);

#[derive(fieldwork::Fieldwork)]
struct D {
    #[fieldwork(ger)]
    field: (),
}

#[derive(fieldwork::Fieldwork)]
struct E {
    #[fieldwork(get(copy, unknown))]
    field: (),
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get(copy, unknown))]
struct F {
    field: (),
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(bounds = true)]
struct G {
    field: (),
}

#[derive(fieldwork::Fieldwork)]
struct H {
    #[fieldwork(get(rename = true))]
    field: (),
}

fn main() {}
