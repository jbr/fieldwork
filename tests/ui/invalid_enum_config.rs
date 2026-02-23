/// Type mismatch with a field-level annotation requesting generation → error.
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set)]
enum TypeMismatchWithAnnotation {
    A { x: i32 },
    B {
        #[field(get)]
        x: String,
    },
}

/// Multiple #[field] annotations on the same virtual field → error.
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get)]
enum MultipleAnnotations {
    A {
        #[field(get)]
        x: i32,
    },
    B {
        #[field(get)]
        x: i32,
    },
}

/// Rename splits into two groups; each group has exactly one annotation → fine.
/// This should compile successfully — no error expected here, but trybuild
/// requires all items to fail, so we comment this case out and test it elsewhere.

fn main() {}
