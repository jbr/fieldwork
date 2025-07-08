use std::borrow::Cow;
use std::fmt::Display;
use std::ops::{Deref, DerefMut};

struct DerefType;
struct OwnedType(DerefType);
impl Deref for OwnedType {
    type Target = DerefType;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for OwnedType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, get_mut, bounds = "T: Clone")]
struct Detection<'a, T: Clone> {
    string: String,
    vec: Vec<u8>,
    boxed: Box<T>,
    boxed_dyn: Box<dyn Display>,
    arc: std::sync::Arc<T>,
    rc: std::rc::Rc<T>,
    cow: Cow<'a, T>,
    path: std::path::PathBuf,
    os_string: std::ffi::OsString,
    arr: [u8; 16],
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
#[fieldwork(get, get_mut, bounds = "T: Clone")]
struct OptionDeref<'a, T: Clone> {
    string: Option<String>,
    vec: Option<Vec<u8>>,
    boxed: Option<Box<T>>,
    boxed_dyn: Option<Box<dyn Display + 'static>>,
    arc: Option<std::sync::Arc<T>>,
    rc: Option<std::rc::Rc<T>>,
    cow: Option<Cow<'a, T>>,
    path: Option<std::path::PathBuf>,
    os_string: Option<std::ffi::OsString>,
    arr: Option<[u8; 16]>,
}

mod x {
    use std::borrow::Cow;
    use std::ops::{Deref, DerefMut};
    use std::path::PathBuf;
    use std::sync::Arc;
    #[derive(fieldwork::Fieldwork)]
    #[fieldwork(get, get_mut, bounds = "T: Deref + DerefMut + Clone")]
    struct OptionMultiDeref<'a, T: Clone> {
        a: Option<std::rc::Rc<PathBuf>>,
        b: Option<Box<Arc<Cow<'a, T>>>>,
        c: Option<Arc<Vec<u8>>>,
        d: Option<Box<Vec<T>>>,
        #[fieldwork(deref = T::Target)]
        e: Option<Box<T>>,
    }
}
