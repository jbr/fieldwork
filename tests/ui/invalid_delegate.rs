#[derive(fieldwork::Fieldwork)]
#[fieldwork(get)]
struct Plain {
    value: (),
}

/// `Plain` isn't `delegatable`, so there's no macro to invoke
#[derive(fieldwork::Fieldwork)]
struct DelegatesToPlain {
    #[field(delegate)]
    plain: Plain,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, delegatable)]
struct Delegatable {
    value: (),
}

/// The delegate can't be inferred from a non-path type
#[derive(fieldwork::Fieldwork)]
struct Uninferrable {
    #[field(delegate)]
    delegatable: &'static Delegatable,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(delegatable = "yes")]
struct NotABool {
    value: (),
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, delegatable)]
enum DelegatableEnum {
    A { value: () },
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get)]
enum DelegateInEnum {
    A {
        #[field(delegate)]
        delegatable: Delegatable,
    },
}

fn main() {}
