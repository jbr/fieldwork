#![allow(dead_code)]

use std::{
    borrow::Cow,
    ops::{Deref, DerefMut},
    path::PathBuf,
    sync::Arc,
};

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, get_mut, bounds = "T: Clone + Deref + DerefMut")]
struct OptionMultiDeref<'a, T: Clone>(
    #[field = "a"] Option<std::rc::Rc<PathBuf>>,
    #[field = "b"] Option<Box<Arc<Cow<'a, T>>>>,
    #[field = "c"] Option<Arc<Vec<u8>>>,
    #[field = "d"] Option<Box<Vec<T>>>,
    #[field(deref = T::Target, name = e)] Option<Box<T>>,
);

fn main() {}
