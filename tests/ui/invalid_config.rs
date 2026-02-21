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

// field-level ExprAssign with non-str/path/bool right-hand side
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get)]
struct T {
    #[fieldwork(name = 1.0)]
    field: (),
}

// field-level ExprCall with unrecognized method name
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get)]
struct U {
    #[fieldwork(bad_method(copy))]
    field: (),
}

// field-level non-assign/path/call expression
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get)]
struct V {
    #[fieldwork("string")]
    field: (),
}

// field method: non-assign/path expression in method args
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get)]
struct W {
    #[fieldwork(get("string"))]
    field: (),
}

// field method: non-str/path/bool right-hand side in method args
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get)]
struct X {
    #[fieldwork(get(name = 1.0))]
    field: (),
}

// item method: non-assign/path expression in method args
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get("string"))]
struct Y {
    field: (),
}

// item method: non-str/bool right-hand side in method args
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get(template = 1.0))]
struct Z {
    field: (),
}

// field method: non-path left-hand side in assignment
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get)]
struct AA {
    #[fieldwork(get((name) = "foo"))]
    field: (),
}

// item-level fieldwork attr that is NameValue, not List
#[derive(fieldwork::Fieldwork)]
#[fieldwork = "not_a_list"]
struct BB {
    field: (),
}

// item-level literal expression (not assign/path/call) in fieldwork list
#[derive(fieldwork::Fieldwork)]
#[fieldwork("string")]
struct CC {
    field: (),
}

// item-level assign with non-str/bool right-hand side
#[derive(fieldwork::Fieldwork)]
#[fieldwork(skip = 1.0)]
struct DD {
    field: (),
}

// item-level method: non-path left-hand side in assignment
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get((template) = "foo"))]
struct EE {
    field: (),
}

// derive on a union is unsupported
#[derive(fieldwork::Fieldwork)]
union FF {
    x: i32,
    y: f32,
}

fn main() {}
