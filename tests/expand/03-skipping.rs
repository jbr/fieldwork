#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set, with, get_mut)]
struct MyStruct<T> {
    /// this number is cool
    number: usize,
    /// is this struct on or not
    #[fieldwork(skip)]
    enabled: bool,
    /// it's really whatever you want
    #[fieldwork(skip = true)]
    generic: T,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set, with, get_mut)]
struct AnotherInterface<T> {
    number: usize,
    #[fieldwork = false]
    enabled: bool,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(set, get)]
struct SetAndGet<T> {
    /// this number is cool
    number: usize,
    /// is this struct on or not
    #[fieldwork(get(skip = true))]
    enabled: bool,
    /// it's really whatever you want
    #[fieldwork(set(skip))]
    generic: T,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set, with, get_mut)]
struct SkipWithAssignment<T> {
    #[fieldwork(with = false)]
    no_with: bool,
    #[fieldwork(get = false)]
    no_get: T,
}

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, get_mut = false)] // parses just fine, but noops
struct GetMutEqualsFalseDoesNothing<T> {
    field: T,
}
