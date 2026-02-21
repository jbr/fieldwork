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
impl<'a, T: Clone> Detection<'a, T>
where
    T: Clone,
{
    pub fn string(&self) -> &str {
        &*self.string
    }
    pub fn string_mut(&mut self) -> &mut str {
        &mut *self.string
    }
    pub fn vec(&self) -> &[u8] {
        &*self.vec
    }
    pub fn vec_mut(&mut self) -> &mut [u8] {
        &mut *self.vec
    }
    pub fn boxed(&self) -> &T {
        &*self.boxed
    }
    pub fn boxed_mut(&mut self) -> &mut T {
        &mut *self.boxed
    }
    pub fn boxed_dyn(&self) -> &dyn Display {
        &*self.boxed_dyn
    }
    pub fn boxed_dyn_mut(&mut self) -> &mut dyn Display {
        &mut *self.boxed_dyn
    }
    pub fn arc(&self) -> &T {
        &*self.arc
    }
    pub fn arc_mut(&mut self) -> &mut std::sync::Arc<T> {
        &mut self.arc
    }
    pub fn rc(&self) -> &T {
        &*self.rc
    }
    pub fn rc_mut(&mut self) -> &mut std::rc::Rc<T> {
        &mut self.rc
    }
    pub fn cow(&self) -> &T {
        &*self.cow
    }
    pub fn cow_mut(&mut self) -> &mut Cow<'a, T> {
        &mut self.cow
    }
    pub fn path(&self) -> &std::path::Path {
        &*self.path
    }
    pub fn path_mut(&mut self) -> &mut std::path::Path {
        &mut *self.path
    }
    pub fn os_string(&self) -> &std::ffi::OsStr {
        &*self.os_string
    }
    pub fn os_string_mut(&mut self) -> &mut std::ffi::OsStr {
        &mut *self.os_string
    }
    pub fn arr(&self) -> &[u8] {
        &self.arr[..]
    }
    pub fn arr_mut(&mut self) -> &mut [u8] {
        &mut self.arr[..]
    }
}
#[fieldwork(get, get_mut, deref = false)]
struct DerefFalseAtStruct {
    baseline_no_auto_deref: String,
    #[fieldwork(deref = true)]
    field_deref_true: Vec<u8>,
}
impl DerefFalseAtStruct {
    pub fn baseline_no_auto_deref(&self) -> &String {
        &self.baseline_no_auto_deref
    }
    pub fn baseline_no_auto_deref_mut(&mut self) -> &mut String {
        &mut self.baseline_no_auto_deref
    }
    pub fn field_deref_true(&self) -> &[u8] {
        &*self.field_deref_true
    }
    pub fn field_deref_true_mut(&mut self) -> &mut [u8] {
        &mut *self.field_deref_true
    }
}
#[fieldwork(get, get_mut(deref = false))]
struct DerefFalseForGetMut {
    baseline_deref_for_get_but_not_for_get_mut: Vec<()>,
    #[fieldwork(get_mut(deref))]
    deref_both: String,
    #[fieldwork(deref = true)]
    also_deref_both: Vec<u8>,
}
impl DerefFalseForGetMut {
    pub fn baseline_deref_for_get_but_not_for_get_mut(&self) -> &[()] {
        &*self.baseline_deref_for_get_but_not_for_get_mut
    }
    pub fn baseline_deref_for_get_but_not_for_get_mut_mut(&mut self) -> &mut Vec<()> {
        &mut self.baseline_deref_for_get_but_not_for_get_mut
    }
    pub fn deref_both(&self) -> &str {
        &*self.deref_both
    }
    pub fn deref_both_mut(&mut self) -> &mut str {
        &mut *self.deref_both
    }
    pub fn also_deref_both(&self) -> &[u8] {
        &*self.also_deref_both
    }
    pub fn also_deref_both_mut(&mut self) -> &mut [u8] {
        &mut *self.also_deref_both
    }
}
#[fieldwork(get, get_mut, deref = false)]
struct SpecifyingTypeStillWorksWithDerefFalse {
    baseline_no_auto_deref: Vec<()>,
    #[fieldwork(get(deref = DerefType))]
    deref_get_to_specified_type: OwnedType,
    #[fieldwork(deref = DerefType)]
    deref_to_specified_type: OwnedType,
}
impl SpecifyingTypeStillWorksWithDerefFalse {
    pub fn baseline_no_auto_deref(&self) -> &Vec<()> {
        &self.baseline_no_auto_deref
    }
    pub fn baseline_no_auto_deref_mut(&mut self) -> &mut Vec<()> {
        &mut self.baseline_no_auto_deref
    }
    pub fn deref_get_to_specified_type(&self) -> &DerefType {
        &*self.deref_get_to_specified_type
    }
    pub fn deref_get_to_specified_type_mut(&mut self) -> &mut OwnedType {
        &mut self.deref_get_to_specified_type
    }
    pub fn deref_to_specified_type(&self) -> &DerefType {
        &*self.deref_to_specified_type
    }
    pub fn deref_to_specified_type_mut(&mut self) -> &mut DerefType {
        &mut *self.deref_to_specified_type
    }
}
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
impl<'a, T: Clone> OptionDeref<'a, T>
where
    T: Clone,
{
    pub fn string(&self) -> Option<&str> {
        self.string.as_deref()
    }
    pub fn string_mut(&mut self) -> Option<&mut str> {
        self.string.as_deref_mut()
    }
    pub fn vec(&self) -> Option<&[u8]> {
        self.vec.as_deref()
    }
    pub fn vec_mut(&mut self) -> Option<&mut [u8]> {
        self.vec.as_deref_mut()
    }
    pub fn boxed(&self) -> Option<&T> {
        self.boxed.as_deref()
    }
    pub fn boxed_mut(&mut self) -> Option<&mut T> {
        self.boxed.as_deref_mut()
    }
    pub fn boxed_dyn(&self) -> Option<&(dyn Display + 'static)> {
        self.boxed_dyn.as_deref()
    }
    pub fn boxed_dyn_mut(&mut self) -> Option<&mut (dyn Display + 'static)> {
        self.boxed_dyn.as_deref_mut()
    }
    pub fn arc(&self) -> Option<&T> {
        self.arc.as_deref()
    }
    pub fn arc_mut(&mut self) -> Option<&mut std::sync::Arc<T>> {
        self.arc.as_mut()
    }
    pub fn rc(&self) -> Option<&T> {
        self.rc.as_deref()
    }
    pub fn rc_mut(&mut self) -> Option<&mut std::rc::Rc<T>> {
        self.rc.as_mut()
    }
    pub fn cow(&self) -> Option<&T> {
        self.cow.as_deref()
    }
    pub fn cow_mut(&mut self) -> Option<&mut Cow<'a, T>> {
        self.cow.as_mut()
    }
    pub fn path(&self) -> Option<&std::path::Path> {
        self.path.as_deref()
    }
    pub fn path_mut(&mut self) -> Option<&mut std::path::Path> {
        self.path.as_deref_mut()
    }
    pub fn os_string(&self) -> Option<&std::ffi::OsStr> {
        self.os_string.as_deref()
    }
    pub fn os_string_mut(&mut self) -> Option<&mut std::ffi::OsStr> {
        self.os_string.as_deref_mut()
    }
    pub fn arr(&self) -> Option<&[u8]> {
        self.arr.as_ref().map(|arr| &arr[..])
    }
    pub fn arr_mut(&mut self) -> Option<&mut [u8]> {
        self.arr.as_mut().map(|arr| &mut arr[..])
    }
}
mod x {
    use std::borrow::Cow;
    use std::ops::{Deref, DerefMut};
    use std::path::PathBuf;
    use std::sync::Arc;
    #[fieldwork(get, get_mut, bounds = "T: Deref + DerefMut + Clone")]
    struct OptionMultiDeref<'a, T: Clone> {
        a: Option<std::rc::Rc<PathBuf>>,
        b: Option<Box<Arc<Cow<'a, T>>>>,
        c: Option<Arc<Vec<u8>>>,
        d: Option<Box<Vec<T>>>,
        #[fieldwork(deref = T::Target)]
        e: Option<Box<T>>,
    }
    impl<'a, T: Clone> OptionMultiDeref<'a, T>
    where
        T: Deref + DerefMut + Clone,
    {
        pub fn a(&self) -> Option<&std::path::Path> {
            self.a.as_ref().map(|a| &***a)
        }
        pub fn a_mut(&mut self) -> Option<&mut std::rc::Rc<PathBuf>> {
            self.a.as_mut()
        }
        pub fn b(&self) -> Option<&T> {
            self.b.as_ref().map(|b| &****b)
        }
        pub fn b_mut(&mut self) -> Option<&mut Arc<Cow<'a, T>>> {
            self.b.as_deref_mut()
        }
        pub fn c(&self) -> Option<&[u8]> {
            self.c.as_ref().map(|c| &***c)
        }
        pub fn c_mut(&mut self) -> Option<&mut Arc<Vec<u8>>> {
            self.c.as_mut()
        }
        pub fn d(&self) -> Option<&[T]> {
            self.d.as_ref().map(|d| &***d)
        }
        pub fn d_mut(&mut self) -> Option<&mut [T]> {
            self.d.as_mut().map(|d| &mut ***d)
        }
        pub fn e(&self) -> Option<&T::Target> {
            self.e.as_ref().map(|e| &***e)
        }
        pub fn e_mut(&mut self) -> Option<&mut T::Target> {
            self.e.as_mut().map(|e| &mut ***e)
        }
    }
}
/// Enum: auto-deref on full-coverage String fields → &str
#[fieldwork(get, get_mut)]
enum AutoDerefEnum {
    First { name: String, path: std::path::PathBuf },
    Second { name: String, path: std::path::PathBuf },
}
impl AutoDerefEnum {
    pub fn name(&self) -> &str {
        match self {
            Self::First { name, .. } | Self::Second { name, .. } => &**name,
        }
    }
    pub fn name_mut(&mut self) -> &mut str {
        match self {
            Self::First { name, .. } => &mut **name,
            Self::Second { name, .. } => &mut **name,
        }
    }
    pub fn path(&self) -> &std::path::Path {
        match self {
            Self::First { path, .. } | Self::Second { path, .. } => &**path,
        }
    }
    pub fn path_mut(&mut self) -> &mut std::path::Path {
        match self {
            Self::First { path, .. } => &mut **path,
            Self::Second { path, .. } => &mut **path,
        }
    }
}
/// Enum: auto-deref with partial coverage — Option wraps the borrowed form
#[fieldwork(get)]
enum PartialAutoDeref {
    WithLabel { id: u32, label: String },
    WithoutLabel { id: u32 },
}
impl PartialAutoDeref {
    pub fn id(&self) -> u32 {
        match self {
            Self::WithLabel { id, .. } | Self::WithoutLabel { id, .. } => *id,
        }
    }
    pub fn label(&self) -> Option<&str> {
        match self {
            Self::WithLabel { label, .. } => Some(&**label),
            _ => None,
        }
    }
}
/// Enum: deref = false disables auto-deref on enum fields
#[fieldwork(get, get_mut, deref = false)]
enum NoAutoDeref {
    A { name: String },
    B { name: String },
}
impl NoAutoDeref {
    pub fn name(&self) -> &String {
        match self {
            Self::A { name, .. } | Self::B { name, .. } => name,
        }
    }
    pub fn name_mut(&mut self) -> &mut String {
        match self {
            Self::A { name, .. } => name,
            Self::B { name, .. } => name,
        }
    }
}
/// Enum: multi-level deref through Option (full and partial coverage)
/// Option<Vec<u8>> → as_deref() → Option<&[u8]>  (single deref level)
/// Option<std::sync::Arc<Vec<u8>>> → as_ref().map() → Option<&[u8]>  (two deref levels)
#[fieldwork(get, get_mut)]
enum DeepDeref {
    Full { single: Option<Vec<u8>>, multi: Option<std::sync::Arc<Vec<u8>>> },
    Partial { single: Option<Vec<u8>> },
}
impl DeepDeref {
    pub fn multi(&self) -> Option<&[u8]> {
        match self {
            Self::Full { multi, .. } => multi.as_ref().map(|multi| &***multi),
            _ => None,
        }
    }
    pub fn multi_mut(&mut self) -> Option<&mut std::sync::Arc<Vec<u8>>> {
        match self {
            Self::Full { multi, .. } => multi.as_mut(),
            _ => None,
        }
    }
    pub fn single(&self) -> Option<&[u8]> {
        match self {
            Self::Full { single, .. } | Self::Partial { single, .. } => single.as_deref(),
        }
    }
    pub fn single_mut(&mut self) -> Option<&mut [u8]> {
        match self {
            Self::Full { single, .. } => single.as_deref_mut(),
            Self::Partial { single, .. } => single.as_deref_mut(),
        }
    }
}
