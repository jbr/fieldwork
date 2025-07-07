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
    pub fn red(&self) -> u8 {
        self.0
    }
    pub fn red_mut(&mut self) -> &mut u8 {
        &mut self.0
    }
    pub fn set_red(&mut self, red: u8) -> &mut Self {
        self.0 = red;
        self
    }
    #[must_use]
    pub fn with_red(mut self, red: u8) -> Self {
        self.0 = red;
        self
    }
    pub fn blue(&self) -> u8 {
        self.1
    }
    pub fn blue_mut(&mut self) -> &mut u8 {
        &mut self.1
    }
    pub fn set_blue(&mut self, blue: u8) -> &mut Self {
        self.1 = blue;
        self
    }
    #[must_use]
    pub fn with_blue(mut self, blue: u8) -> Self {
        self.1 = blue;
        self
    }
    pub fn green(&self) -> u8 {
        self.2
    }
    pub fn green_mut(&mut self) -> &mut u8 {
        &mut self.2
    }
    pub fn set_green(&mut self, green: u8) -> &mut Self {
        self.2 = green;
        self
    }
    #[must_use]
    pub fn with_green(mut self, green: u8) -> Self {
        self.2 = green;
        self
    }
}
#[fieldwork(get, set, with, get_mut, option_set_some)]
struct Color(#[fieldwork(name = rgb, copy)] Rgb, #[fieldwork(name = alpha)] Option<u8>);
impl Color {
    pub fn rgb(&self) -> Rgb {
        self.0
    }
    pub fn rgb_mut(&mut self) -> &mut Rgb {
        &mut self.0
    }
    pub fn set_rgb(&mut self, rgb: Rgb) -> &mut Self {
        self.0 = rgb;
        self
    }
    #[must_use]
    pub fn with_rgb(mut self, rgb: Rgb) -> Self {
        self.0 = rgb;
        self
    }
    pub fn alpha(&self) -> Option<u8> {
        self.1
    }
    pub fn alpha_mut(&mut self) -> Option<&mut u8> {
        self.1.as_mut()
    }
    pub fn set_alpha(&mut self, alpha: u8) -> &mut Self {
        self.1 = Some(alpha);
        self
    }
    #[must_use]
    pub fn with_alpha(mut self, alpha: u8) -> Self {
        self.1 = Some(alpha);
        self
    }
}
#[fieldwork(get, set, with, get_mut, option_set_some)]
struct OneFieldSkipped(String, #[fieldwork(name = name)] Option<String>);
impl OneFieldSkipped {
    pub fn name(&self) -> Option<&str> {
        self.1.as_deref()
    }
    pub fn name_mut(&mut self) -> Option<&mut str> {
        self.1.as_deref_mut()
    }
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.1 = Some(name);
        self
    }
    #[must_use]
    pub fn with_name(mut self, name: String) -> Self {
        self.1 = Some(name);
        self
    }
}
#[fieldwork(get, set, with, get_mut, option_set_some)]
struct OnlyGet(String, #[fieldwork(get(name = name))] Option<String>);
impl OnlyGet {
    pub fn name(&self) -> Option<&str> {
        self.1.as_deref()
    }
}
#[fieldwork(get, get_mut)]
struct OptionMultiDeref<'a, T>(
    #[field = "a"]
    Option<std::rc::Rc<PathBuf>>,
    #[field = "b"]
    Option<Box<Arc<Cow<'a, T>>>>,
    #[field = "c"]
    Option<Arc<Vec<u8>>>,
    #[field = "d"]
    Option<Box<Vec<T>>>,
    #[field(deref = U, name = e)]
    Option<Box<T>>,
);
impl<'a, T> OptionMultiDeref<'a, T> {
    pub fn a(&self) -> Option<&std::path::Path> {
        self.0.as_ref().map(|a| &**a)
    }
    pub fn a_mut(&mut self) -> Option<&mut std::rc::Rc<PathBuf>> {
        self.0.as_mut()
    }
    pub fn b(&self) -> Option<&T> {
        self.1.as_ref().map(|b| &***b)
    }
    pub fn b_mut(&mut self) -> Option<&mut Arc<Cow<'a, T>>> {
        self.1.as_deref_mut()
    }
    pub fn c(&self) -> Option<&[u8]> {
        self.2.as_ref().map(|c| &**c)
    }
    pub fn c_mut(&mut self) -> Option<&mut Arc<Vec<u8>>> {
        self.2.as_mut()
    }
    pub fn d(&self) -> Option<&[T]> {
        self.3.as_ref().map(|d| &**d)
    }
    pub fn d_mut(&mut self) -> Option<&mut [T]> {
        self.3.as_mut().map(|d| &mut **d)
    }
    pub fn e(&self) -> Option<&U> {
        self.4.as_ref().map(|e| &**e)
    }
    pub fn e_mut(&mut self) -> Option<&mut U> {
        self.4.as_mut().map(|e| &mut **e)
    }
}
