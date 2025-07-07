#[derive(fieldwork::Fieldwork)]
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
#[fieldwork(get, get_mut)]
struct OptionMultiDeref<'a, T>(
    #[fieldwork(name = a)] Option<std::rc::Rc<PathBuf>>,
    #[fieldwork(name = b)] Option<Box<Arc<Cow<'a, T>>>>,
    #[fieldwork(name = c)] Option<Arc<Vec<u8>>>,
    #[fieldwork(name = d)] Option<Box<Vec<T>>>,
    #[fieldwork(deref = U, name = e)] Option<Box<T>>,
);
