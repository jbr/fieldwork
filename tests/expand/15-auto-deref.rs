#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, get_mut)]
struct Detection<'a, T> {
    string: String,
    vec: Vec<u8>,
    boxed: Box<T>,
    boxed_dyn: Box<dyn Display>,
    arc: std::sync::Arc<T>,
    rc: std::rc::Rc<T>,
    cow: Cow<'a, T>,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, get_mut, deref = false)]
struct DerefFalseAtStruct {
    baseline_no_auto_deref: String,

    #[fieldwork(deref = true)]
    field_deref_true: Vec<u8>,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, get_mut(deref = false))]
struct DerefFalseForGetMut {
    baseline_deref_for_get_but_not_for_get_mut: Vec<()>,

    #[fieldwork(get_mut(deref))]
    deref_both: String,

    #[fieldwork(deref = true)]
    also_deref_both: Vec<u8>,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, get_mut, deref = false)]
struct SpecifyingTypeStillWorksWithDerefFalse {
    baseline_no_auto_deref: Vec<()>,

    #[fieldwork(get(deref = DerefType))]
    deref_get_to_specified_type: OwnedType,

    #[fieldwork(deref = DerefType)]
    deref_to_specified_type: OwnedType,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, get_mut, deref = false)]
struct SpecifyingTypeStillWorksWithDerefFalse {
    baseline_no_auto_deref: Vec<()>,

    #[fieldwork(get(deref = DerefType))]
    deref_get_to_specified_type: OwnedType,

    #[fieldwork(deref = DerefType)]
    deref_to_specified_type: OwnedType,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, get_mut)]
struct OptionDeref<'a, T> {
    string: Option<String>,
    vec: Option<Vec<u8>>,
    boxed: Option<Box<T>>,
    boxed_dyn: Option<Box<dyn Display>>,
    arc: Option<std::sync::Arc<T>>,
    rc: Option<std::rc::Rc<T>>,
    cow: Option<Cow<'a, T>>,
}
