#[derive(fieldwork::Fieldwork)]
#[fieldwork(set)]
struct MyStruct<T> {
    number: usize,

    /// opting out
    #[fieldwork(set(chain = false))]
    enabled: bool,

    generic: T,
}
#[derive(fieldwork::Fieldwork)]
#[fieldwork(set(chain = false))]
struct MyStruct2<T> {
    /// opted out at struct-method level
    number: usize,

    #[fieldwork(set(chain = true))]
    /// opting back in
    enabled: bool,

    #[fieldwork(set(chain))]
    /// opting back in
    generic: T,
}
