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

#[derive(fieldwork::Fieldwork)]
#[fieldwork(invalid(copy))]
struct I {
    field: (),
}

#[derive(fieldwork::Fieldwork)]
struct J {
    #[fieldwork(get(other = "anything"))]
    field: (),
}

#[derive(fieldwork::Fieldwork)]
struct K {
    #[fieldwork(get(other = anything))]
    field: (),
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get(other = "anything"))]
struct L {
    field: (),
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(other = "anything")]
struct M {
    field: (),
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork((get)(copy))]
struct O(());

#[derive(fieldwork::Fieldwork)]
#[fieldwork((get) = copy)]
struct P(());

#[derive(fieldwork::Fieldwork)]
struct Q(#[fieldwork((get) = copy)] ());

#[derive(fieldwork::Fieldwork)]
struct R(#[fieldwork((get)(copy))] ());

#[derive(fieldwork::Fieldwork)]
struct S(#[fieldwork = 1.0] ());

fn main() {}
