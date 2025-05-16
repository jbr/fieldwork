#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set, get_mut, with)]
struct MyStruct<T> {
    /// the number
    #[fieldwork(argument = the_number)]
    number: usize,
    /// whether something is enabled
    #[fieldwork(set(argument = "is_enabled_as_a_boolean"))]
    enabled: bool,
    /// the generic
    #[fieldwork(argument = "the_gen", set(argument = the_generic))]
    generic: T,
}
