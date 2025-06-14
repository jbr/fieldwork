#[fieldwork(get, set, get_mut, with)]
struct User {
    /// whether this user is an admin
    ///
    /// Note that this is distinct from the notion of group administration,
    /// for historical reasons
    #[fieldwork(argument = is_admin, get = is_admin, get_mut = is_admin_mut)]
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
impl User {
    /**Returns a copy of whether this user is an admin

Note that this is distinct from the notion of group administration,
for historical reasons*/
    pub fn is_admin(&self) -> bool {
        self.admin
    }
    /**Mutably borrow whether this user is an admin

Note that this is distinct from the notion of group administration,
for historical reasons*/
    pub fn is_admin_mut(&mut self) -> &mut bool {
        &mut self.admin
    }
    /**Sets whether this user is an admin, returning `&mut Self` for chaining

Note that this is distinct from the notion of group administration,
for historical reasons*/
    pub fn set_admin(&mut self, is_admin: bool) -> &mut Self {
        self.admin = is_admin;
        self
    }
    /**Owned chainable setter for whether this user is an admin, returning `Self`

Note that this is distinct from the notion of group administration,
for historical reasons*/
    #[must_use]
    pub fn with_admin(mut self, is_admin: bool) -> Self {
        self.admin = is_admin;
        self
    }
    ///Borrows the user's name
    pub fn name(&self) -> &str {
        &*self.name
    }
    ///Mutably borrow the user's name
    pub fn name_mut(&mut self) -> &mut str {
        &mut *self.name
    }
    ///Sets the user's name, returning `&mut Self` for chaining
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }
    ///Owned chainable setter for the user's name, returning `Self`
    #[must_use]
    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
    ///Borrows the user's favorite color, if set
    pub fn favorite_color(&self) -> Option<&str> {
        self.favorite_color.as_deref()
    }
    ///Mutably borrow the user's favorite color, if set
    pub fn favorite_color_mut(&mut self) -> Option<&mut str> {
        self.favorite_color.as_deref_mut()
    }
    ///Sets the user's favorite color, if set, returning `&mut Self` for chaining
    pub fn set_favorite_color(&mut self, favorite_color: Option<String>) -> &mut Self {
        self.favorite_color = favorite_color;
        self
    }
    ///Owned chainable setter for the user's favorite color, if set, returning `Self`
    #[must_use]
    pub fn with_favorite_color(mut self, favorite_color: Option<String>) -> Self {
        self.favorite_color = favorite_color;
        self
    }
    ///Borrows read-only unique identifier
    pub fn id(&self) -> &[u8] {
        &*self.id
    }
}
