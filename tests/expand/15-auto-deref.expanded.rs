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
impl<'a, T> Detection<'a, T> {
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
    pub fn arc_mut(&mut self) -> &mut T {
        &mut *self.arc
    }
    pub fn rc(&self) -> &T {
        &*self.rc
    }
    pub fn rc_mut(&mut self) -> &mut T {
        &mut *self.rc
    }
    pub fn cow(&self) -> &T {
        &*self.cow
    }
    pub fn cow_mut(&mut self) -> &mut T {
        &mut *self.cow
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
impl<'a, T> OptionDeref<'a, T> {
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
    pub fn boxed_dyn(&self) -> Option<&dyn Display> {
        self.boxed_dyn.as_deref()
    }
    pub fn boxed_dyn_mut(&mut self) -> Option<&mut dyn Display> {
        self.boxed_dyn.as_deref_mut()
    }
    pub fn arc(&self) -> Option<&T> {
        self.arc.as_deref()
    }
    pub fn arc_mut(&mut self) -> Option<&mut T> {
        self.arc.as_deref_mut()
    }
    pub fn rc(&self) -> Option<&T> {
        self.rc.as_deref()
    }
    pub fn rc_mut(&mut self) -> Option<&mut T> {
        self.rc.as_deref_mut()
    }
    pub fn cow(&self) -> Option<&T> {
        self.cow.as_deref()
    }
    pub fn cow_mut(&mut self) -> Option<&mut T> {
        self.cow.as_deref_mut()
    }
}
