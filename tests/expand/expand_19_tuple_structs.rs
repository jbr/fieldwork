use std::borrow::Cow;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::sync::Arc;

#[derive(fieldwork::Fieldwork, Clone, Copy)]
#[fieldwork(get, set, with, get_mut)]
struct Rgb(
    #[fieldwork(name = red)] u8,
    #[fieldwork(name = blue)] u8,
    #[fieldwork(name = green)] u8,
);

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set, with, get_mut, option_set_some)]
struct Color(
    #[fieldwork(name = rgb, copy)] Rgb,
    #[fieldwork(name = alpha)] Option<u8>,
);

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set, with, get_mut, option_set_some)]
struct OneFieldSkipped(String, #[fieldwork(name = name)] Option<String>);

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set, with, get_mut, option_set_some)]
struct OnlyGet(String, #[fieldwork(get(name = name))] Option<String>);

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, get_mut, bounds = "T: Clone + Deref + DerefMut")]
struct OptionMultiDeref<'a, T: Clone>(
    #[field = "a"] Option<std::rc::Rc<PathBuf>>,
    #[field = "b"] Option<Box<Arc<Cow<'a, T>>>>,
    #[field = "c"] Option<Arc<Vec<u8>>>,
    #[field = "d"] Option<Box<Vec<T>>>,
    #[field(deref = T::Target, name = e)] Option<Box<T>>,
);
