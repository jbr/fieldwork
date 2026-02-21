struct DerefType;
struct OwnedType(DerefType);
impl std::ops::Deref for OwnedType {
    type Target = DerefType;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for OwnedType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, get_mut)]
struct OptionBehavior {
    #[fieldwork(deref = "Option<&str>")]
    option_deref: Option<String>,

    #[fieldwork(deref = str)]
    option_deref_other_style: Option<String>,

    #[fieldwork(option_borrow_inner = false)]
    no_option_detection: Option<String>,

    option_detection: Option<()>,

    #[fieldwork(get_mut(option_borrow_inner = false))]
    nothing_fancy_for_get_mut: Option<()>,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, get_mut, option_borrow_inner = false)]
struct OptInOption {
    #[fieldwork(deref = "Option<&str>", option_borrow_inner = true)]
    option_deref: Option<String>,

    #[fieldwork(deref = str, option_borrow_inner = true)]
    option_deref_other_style: Option<String>,

    #[fieldwork(option_borrow_inner)]
    option_detection: Option<String>,

    no_option_detection: Option<()>,

    #[fieldwork(get_mut(option_borrow_inner = true))]
    option_detection_only_get_mut: Option<()>,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get(option_borrow_inner = true), get_mut, option_borrow_inner = false)]
struct OptionOnlyForGet {
    #[fieldwork(get(option_borrow_inner = false))]
    no_option_detection: Option<()>,

    #[fieldwork(get(deref = str))]
    option_deref: Option<String>,

    #[fieldwork(option_borrow_inner = true)]
    field_overrides: Option<String>,

    option_only_for_get: Option<()>,

    #[fieldwork(get_mut(option_borrow_inner), get(option_borrow_inner = false))]
    option_detection_only_get_mut: Option<()>,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, get_mut, option_borrow_inner = false, deref = false)]
struct OptionAndDerefInteraction {
    a: Option<String>,
    #[fieldwork(option_borrow_inner)]
    b: Option<String>,
    #[fieldwork(option_borrow_inner, deref)]
    c: Option<String>,
    #[fieldwork(deref)]
    d: Option<String>, // remains Option<String>
    #[fieldwork(option_borrow_inner, deref = "Option<&DerefType>")]
    e: Option<OwnedType>,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, option = false)]
struct BackwardsCompat {
    #[fieldwork(option)]
    borrow_inner: Option<()>,

    not_borrow_inner: Option<String>,
}

/// Enum: Option fields with full coverage → Option<&T> (borrow inner applied)
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, get_mut)]
enum WithOptions {
    Foo { token: Option<String>, id: u32 },
    Bar { token: Option<String> },
}

/// Enum: option_borrow_inner = false disables unwrapping
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, get_mut, option_borrow_inner = false)]
enum NoOptionDetection {
    Foo { data: Option<String> },
    Bar { data: Option<String> },
}

/// Enum: opt-in option detection per field
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, option_borrow_inner = false)]
enum SelectiveOption {
    Foo {
        #[fieldwork(option_borrow_inner)]
        detected: Option<String>,
        raw: Option<String>,
    },
    Bar {
        #[fieldwork(option_borrow_inner)]
        detected: Option<String>,
        raw: Option<String>,
    },
}
