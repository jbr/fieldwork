use std::borrow::Cow;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::sync::Arc;
/// Enum: tuple variants with named fields via #[field = "name"]
#[fieldwork(get, into_field)]
enum ColorEnum {
    Rgb(#[field = "r"] u8, #[field = "g"] u8, #[field = "b"] u8),
    Named(#[field = "name"] String),
    Transparent,
}
impl ColorEnum {
    #[inline]
    pub fn b(&self) -> Option<u8> {
        match self {
            Self::Rgb(_, _, b, ..) => Some(*b),
            _ => None,
        }
    }
    #[inline]
    pub fn g(&self) -> Option<u8> {
        match self {
            Self::Rgb(_, g, ..) => Some(*g),
            _ => None,
        }
    }
    #[inline]
    pub fn name(&self) -> Option<&str> {
        match self {
            Self::Named(name, ..) => Some(&**name),
            _ => None,
        }
    }
    #[inline]
    pub fn r(&self) -> Option<u8> {
        match self {
            Self::Rgb(r, ..) => Some(*r),
            _ => None,
        }
    }
}
/// Enum: mixed named-field and tuple variants
#[fieldwork(get, set)]
enum Payload {
    Tuple(#[field = "data"] Vec<u8>, #[field = "id"] u32),
    Named { data: Vec<u8>, id: u32 },
    Empty,
}
impl Payload {
    #[inline]
    pub fn data(&self) -> Option<&[u8]> {
        match self {
            Self::Tuple(data, ..) => Some(&**data),
            Self::Named { data, .. } => Some(&**data),
            _ => None,
        }
    }
    #[inline]
    pub fn id(&self) -> Option<u32> {
        match self {
            Self::Tuple(_, id, ..) => Some(*id),
            Self::Named { id, .. } => Some(*id),
            _ => None,
        }
    }
}
#[fieldwork(get, set, with, get_mut)]
struct Rgb(
    #[fieldwork(name = red)]
    u8,
    #[fieldwork(name = blue)]
    u8,
    #[fieldwork(name = green)]
    u8,
);
impl Rgb {
    #[inline]
    pub fn red(&self) -> u8 {
        self.0
    }
    #[inline]
    pub fn red_mut(&mut self) -> &mut u8 {
        &mut self.0
    }
    #[inline]
    pub fn set_red(&mut self, red: u8) -> &mut Self {
        self.0 = red;
        self
    }
    #[inline]
    #[must_use]
    pub fn with_red(mut self, red: u8) -> Self {
        self.0 = red;
        self
    }
    #[inline]
    pub fn blue(&self) -> u8 {
        self.1
    }
    #[inline]
    pub fn blue_mut(&mut self) -> &mut u8 {
        &mut self.1
    }
    #[inline]
    pub fn set_blue(&mut self, blue: u8) -> &mut Self {
        self.1 = blue;
        self
    }
    #[inline]
    #[must_use]
    pub fn with_blue(mut self, blue: u8) -> Self {
        self.1 = blue;
        self
    }
    #[inline]
    pub fn green(&self) -> u8 {
        self.2
    }
    #[inline]
    pub fn green_mut(&mut self) -> &mut u8 {
        &mut self.2
    }
    #[inline]
    pub fn set_green(&mut self, green: u8) -> &mut Self {
        self.2 = green;
        self
    }
    #[inline]
    #[must_use]
    pub fn with_green(mut self, green: u8) -> Self {
        self.2 = green;
        self
    }
}
/// Implemented by hand rather than derived because `derive(Clone)` expansion
/// differs between stable and nightly toolchains.
impl Clone for Rgb {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for Rgb {}
#[fieldwork(get, set, with, get_mut, option_set_some)]
struct Color(#[fieldwork(name = rgb, copy)] Rgb, #[fieldwork(name = alpha)] Option<u8>);
impl Color {
    #[inline]
    pub fn rgb(&self) -> Rgb {
        self.0
    }
    #[inline]
    pub fn rgb_mut(&mut self) -> &mut Rgb {
        &mut self.0
    }
    #[inline]
    pub fn set_rgb(&mut self, rgb: Rgb) -> &mut Self {
        self.0 = rgb;
        self
    }
    #[inline]
    #[must_use]
    pub fn with_rgb(mut self, rgb: Rgb) -> Self {
        self.0 = rgb;
        self
    }
    #[inline]
    pub fn alpha(&self) -> Option<u8> {
        self.1
    }
    #[inline]
    pub fn alpha_mut(&mut self) -> Option<&mut u8> {
        self.1.as_mut()
    }
    #[inline]
    pub fn set_alpha(&mut self, alpha: u8) -> &mut Self {
        self.1 = Some(alpha);
        self
    }
    #[inline]
    #[must_use]
    pub fn with_alpha(mut self, alpha: u8) -> Self {
        self.1 = Some(alpha);
        self
    }
}
#[fieldwork(get, set, with, get_mut, option_set_some)]
struct OneFieldSkipped(String, #[fieldwork(name = name)] Option<String>);
impl OneFieldSkipped {
    #[inline]
    pub fn name(&self) -> Option<&str> {
        self.1.as_deref()
    }
    #[inline]
    pub fn name_mut(&mut self) -> Option<&mut str> {
        self.1.as_deref_mut()
    }
    #[inline]
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.1 = Some(name);
        self
    }
    #[inline]
    #[must_use]
    pub fn with_name(mut self, name: String) -> Self {
        self.1 = Some(name);
        self
    }
}
#[fieldwork(get, set, with, get_mut, option_set_some)]
struct OnlyGet(String, #[fieldwork(get(name = name))] Option<String>);
impl OnlyGet {
    #[inline]
    pub fn name(&self) -> Option<&str> {
        self.1.as_deref()
    }
}
#[fieldwork(get, get_mut, bounds = "T: Clone + Deref + DerefMut")]
struct OptionMultiDeref<'a, T: Clone>(
    #[field = "a"]
    Option<std::rc::Rc<PathBuf>>,
    #[field = "b"]
    Option<Box<Arc<Cow<'a, T>>>>,
    #[field = "c"]
    Option<Arc<Vec<u8>>>,
    #[field = "d"]
    Option<Box<Vec<T>>>,
    #[field(deref = T::Target, name = e)]
    Option<Box<T>>,
);
impl<'a, T: Clone> OptionMultiDeref<'a, T>
where
    T: Clone + Deref + DerefMut,
{
    #[inline]
    pub fn a(&self) -> Option<&std::path::Path> {
        self.0.as_ref().map(|a| &***a)
    }
    #[inline]
    pub fn a_mut(&mut self) -> Option<&mut std::rc::Rc<PathBuf>> {
        self.0.as_mut()
    }
    #[inline]
    pub fn b(&self) -> Option<&T> {
        self.1.as_ref().map(|b| &****b)
    }
    #[inline]
    pub fn b_mut(&mut self) -> Option<&mut Arc<Cow<'a, T>>> {
        self.1.as_deref_mut()
    }
    #[inline]
    pub fn c(&self) -> Option<&[u8]> {
        self.2.as_ref().map(|c| &***c)
    }
    #[inline]
    pub fn c_mut(&mut self) -> Option<&mut Arc<Vec<u8>>> {
        self.2.as_mut()
    }
    #[inline]
    pub fn d(&self) -> Option<&[T]> {
        self.3.as_ref().map(|d| &***d)
    }
    #[inline]
    pub fn d_mut(&mut self) -> Option<&mut [T]> {
        self.3.as_mut().map(|d| &mut ***d)
    }
    #[inline]
    pub fn e(&self) -> Option<&T::Target> {
        self.4.as_ref().map(|e| &***e)
    }
    #[inline]
    pub fn e_mut(&mut self) -> Option<&mut T::Target> {
        self.4.as_mut().map(|e| &mut ***e)
    }
}
