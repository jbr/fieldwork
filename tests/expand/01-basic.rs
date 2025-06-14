#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set, get_mut, with)]
struct User {
    /// whether this user is an admin
    ///
    /// Note that this is distinct from the notion of group administration,
    /// for historical reasons
    #[fieldwork(
        argument = is_admin,
        get = is_admin,
        get_mut = is_admin_mut
    )]
    admin: bool,

    /// the user's name
    name: String,

    /// the user's favorite color, if set
    favorite_color: Option<String>,

    #[fieldwork(skip)]
    private: (),

    /// read-only unique identifier
    #[fieldwork(opt_in, get)]
    id: Vec<u8>,
}
