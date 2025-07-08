#![allow(dead_code)]
mod expand_01_basic {
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
        pub fn set_favorite_color(
            &mut self,
            favorite_color: Option<String>,
        ) -> &mut Self {
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
}
mod expand_02_only_set_and_get {
    #[fieldwork(set, get = true)]
    struct MyStruct<T> {
        /// this number is cool
        number: usize,
        /// is this struct on or not
        enabled: bool,
        /// it's really whatever you want
        generic: T,
    }
    impl<T> MyStruct<T> {
        ///Returns a copy of this number is cool
        pub fn number(&self) -> usize {
            self.number
        }
        ///Sets this number is cool, returning `&mut Self` for chaining
        pub fn set_number(&mut self, number: usize) -> &mut Self {
            self.number = number;
            self
        }
        ///Returns a copy of is this struct on or not
        pub fn enabled(&self) -> bool {
            self.enabled
        }
        ///Sets is this struct on or not, returning `&mut Self` for chaining
        pub fn set_enabled(&mut self, enabled: bool) -> &mut Self {
            self.enabled = enabled;
            self
        }
        ///Borrows it's really whatever you want
        pub fn generic(&self) -> &T {
            &self.generic
        }
        ///Sets it's really whatever you want, returning `&mut Self` for chaining
        pub fn set_generic(&mut self, generic: T) -> &mut Self {
            self.generic = generic;
            self
        }
    }
}
mod expand_03_skipping {
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
    impl<T> MyStruct<T> {
        ///Returns a copy of this number is cool
        pub fn number(&self) -> usize {
            self.number
        }
        ///Mutably borrow this number is cool
        pub fn number_mut(&mut self) -> &mut usize {
            &mut self.number
        }
        ///Sets this number is cool, returning `&mut Self` for chaining
        pub fn set_number(&mut self, number: usize) -> &mut Self {
            self.number = number;
            self
        }
        ///Owned chainable setter for this number is cool, returning `Self`
        #[must_use]
        pub fn with_number(mut self, number: usize) -> Self {
            self.number = number;
            self
        }
    }
    #[fieldwork(get, set, with, get_mut)]
    struct AnotherInterface {
        number: usize,
        #[fieldwork = false]
        enabled: bool,
    }
    impl AnotherInterface {
        pub fn number(&self) -> usize {
            self.number
        }
        pub fn number_mut(&mut self) -> &mut usize {
            &mut self.number
        }
        pub fn set_number(&mut self, number: usize) -> &mut Self {
            self.number = number;
            self
        }
        #[must_use]
        pub fn with_number(mut self, number: usize) -> Self {
            self.number = number;
            self
        }
    }
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
    impl<T> SetAndGet<T> {
        ///Returns a copy of this number is cool
        pub fn number(&self) -> usize {
            self.number
        }
        ///Sets this number is cool, returning `&mut Self` for chaining
        pub fn set_number(&mut self, number: usize) -> &mut Self {
            self.number = number;
            self
        }
        ///Sets is this struct on or not, returning `&mut Self` for chaining
        pub fn set_enabled(&mut self, enabled: bool) -> &mut Self {
            self.enabled = enabled;
            self
        }
        ///Borrows it's really whatever you want
        pub fn generic(&self) -> &T {
            &self.generic
        }
    }
    #[fieldwork(get, set, with, get_mut)]
    struct SkipWithAssignment<T> {
        #[fieldwork(with = false)]
        no_with: bool,
        #[fieldwork(get = false)]
        no_get: T,
    }
    impl<T> SkipWithAssignment<T> {
        pub fn no_with(&self) -> bool {
            self.no_with
        }
        pub fn no_with_mut(&mut self) -> &mut bool {
            &mut self.no_with
        }
        pub fn set_no_with(&mut self, no_with: bool) -> &mut Self {
            self.no_with = no_with;
            self
        }
        pub fn no_get_mut(&mut self) -> &mut T {
            &mut self.no_get
        }
        pub fn set_no_get(&mut self, no_get: T) -> &mut Self {
            self.no_get = no_get;
            self
        }
        #[must_use]
        pub fn with_no_get(mut self, no_get: T) -> Self {
            self.no_get = no_get;
            self
        }
    }
    #[fieldwork(get, get_mut = false)]
    struct GetMutEqualsFalseDoesNothing<T> {
        field: T,
    }
    impl<T> GetMutEqualsFalseDoesNothing<T> {
        pub fn field(&self) -> &T {
            &self.field
        }
    }
}
mod expand_04_renaming {
    #[fieldwork(get, set, with, get_mut)]
    struct MyStruct<T> {
        /// this number is cool
        #[fieldwork(rename = number_in_seconds)]
        number: usize,
        /// is this struct on or not
        enabled: bool,
        /// it's really whatever you want
        #[fieldwork(argument = tee)]
        generic: T,
        #[fieldwork(get = "get_another")]
        another: (),
    }
    impl<T> MyStruct<T> {
        ///Returns a copy of this number is cool
        pub fn number_in_seconds(&self) -> usize {
            self.number
        }
        ///Mutably borrow this number is cool
        pub fn number_in_seconds_mut(&mut self) -> &mut usize {
            &mut self.number
        }
        ///Sets this number is cool, returning `&mut Self` for chaining
        pub fn set_number_in_seconds(&mut self, number_in_seconds: usize) -> &mut Self {
            self.number = number_in_seconds;
            self
        }
        ///Owned chainable setter for this number is cool, returning `Self`
        #[must_use]
        pub fn with_number_in_seconds(mut self, number_in_seconds: usize) -> Self {
            self.number = number_in_seconds;
            self
        }
        ///Returns a copy of is this struct on or not
        pub fn enabled(&self) -> bool {
            self.enabled
        }
        ///Mutably borrow is this struct on or not
        pub fn enabled_mut(&mut self) -> &mut bool {
            &mut self.enabled
        }
        ///Sets is this struct on or not, returning `&mut Self` for chaining
        pub fn set_enabled(&mut self, enabled: bool) -> &mut Self {
            self.enabled = enabled;
            self
        }
        ///Owned chainable setter for is this struct on or not, returning `Self`
        #[must_use]
        pub fn with_enabled(mut self, enabled: bool) -> Self {
            self.enabled = enabled;
            self
        }
        ///Borrows it's really whatever you want
        pub fn generic(&self) -> &T {
            &self.generic
        }
        ///Mutably borrow it's really whatever you want
        pub fn generic_mut(&mut self) -> &mut T {
            &mut self.generic
        }
        ///Sets it's really whatever you want, returning `&mut Self` for chaining
        pub fn set_generic(&mut self, tee: T) -> &mut Self {
            self.generic = tee;
            self
        }
        ///Owned chainable setter for it's really whatever you want, returning `Self`
        #[must_use]
        pub fn with_generic(mut self, tee: T) -> Self {
            self.generic = tee;
            self
        }
        pub fn get_another(&self) -> &() {
            &self.another
        }
        pub fn another_mut(&mut self) -> &mut () {
            &mut self.another
        }
        pub fn set_another(&mut self, another: ()) -> &mut Self {
            self.another = another;
            self
        }
        #[must_use]
        pub fn with_another(mut self, another: ()) -> Self {
            self.another = another;
            self
        }
    }
    #[fieldwork(get(template = "get_{}"), set(template = "put_{}"))]
    struct WithTemplate<T> {
        /// this number is cool
        #[fieldwork(rename = "number_in_seconds")]
        number: usize,
        /// is this struct on or not
        #[fieldwork(get = is_it_enabled)]
        enabled: bool,
        /// it's really whatever you want
        #[fieldwork(set(rename = "put_the_generic"))]
        generic: T,
    }
    impl<T> WithTemplate<T> {
        ///Returns a copy of this number is cool
        pub fn get_number_in_seconds(&self) -> usize {
            self.number
        }
        ///Sets this number is cool, returning `&mut Self` for chaining
        pub fn put_number_in_seconds(&mut self, number_in_seconds: usize) -> &mut Self {
            self.number = number_in_seconds;
            self
        }
        ///Returns a copy of is this struct on or not
        pub fn is_it_enabled(&self) -> bool {
            self.enabled
        }
        ///Sets is this struct on or not, returning `&mut Self` for chaining
        pub fn put_enabled(&mut self, enabled: bool) -> &mut Self {
            self.enabled = enabled;
            self
        }
        ///Borrows it's really whatever you want
        pub fn get_generic(&self) -> &T {
            &self.generic
        }
        ///Sets it's really whatever you want, returning `&mut Self` for chaining
        pub fn put_the_generic(&mut self, generic: T) -> &mut Self {
            self.generic = generic;
            self
        }
    }
    #[fieldwork(get, set, with, get_mut, without)]
    struct RenameWithEquals {
        /// this number is cool
        #[fieldwork = "number_in_seconds"]
        number: usize,
        /// is this struct on or not
        #[field = "setting_enabled"]
        enabled: bool,
    }
    impl RenameWithEquals {
        ///Returns a copy of this number is cool
        pub fn number_in_seconds(&self) -> usize {
            self.number
        }
        ///Mutably borrow this number is cool
        pub fn number_in_seconds_mut(&mut self) -> &mut usize {
            &mut self.number
        }
        ///Sets this number is cool, returning `&mut Self` for chaining
        pub fn set_number_in_seconds(&mut self, number_in_seconds: usize) -> &mut Self {
            self.number = number_in_seconds;
            self
        }
        ///Owned chainable setter for this number is cool, returning `Self`
        #[must_use]
        pub fn with_number_in_seconds(mut self, number_in_seconds: usize) -> Self {
            self.number = number_in_seconds;
            self
        }
        ///Returns a copy of is this struct on or not
        pub fn setting_enabled(&self) -> bool {
            self.enabled
        }
        ///Mutably borrow is this struct on or not
        pub fn setting_enabled_mut(&mut self) -> &mut bool {
            &mut self.enabled
        }
        ///Sets is this struct on or not, returning `&mut Self` for chaining
        pub fn set_setting_enabled(&mut self, setting_enabled: bool) -> &mut Self {
            self.enabled = setting_enabled;
            self
        }
        ///Owned chainable setter for is this struct on or not, returning `Self`
        #[must_use]
        pub fn with_setting_enabled(mut self) -> Self {
            self.enabled = true;
            self
        }
        ///Owned chainable setter for is this struct on or not, returning `Self`
        #[must_use]
        pub fn without_setting_enabled(mut self) -> Self {
            self.enabled = false;
            self
        }
    }
}
mod expand_05_template {
    #[fieldwork(get(template = "get_{}"), set(template = "assign_{}"), with, get_mut)]
    struct MyStruct<T> {
        /// this number is cool
        #[fieldwork(rename = number_in_seconds)]
        number: usize,
        /// is this struct on or not
        enabled: bool,
        /// it's really whatever you want
        generic: T,
    }
    impl<T> MyStruct<T> {
        ///Returns a copy of this number is cool
        pub fn get_number_in_seconds(&self) -> usize {
            self.number
        }
        ///Mutably borrow this number is cool
        pub fn number_in_seconds_mut(&mut self) -> &mut usize {
            &mut self.number
        }
        ///Sets this number is cool, returning `&mut Self` for chaining
        pub fn assign_number_in_seconds(
            &mut self,
            number_in_seconds: usize,
        ) -> &mut Self {
            self.number = number_in_seconds;
            self
        }
        ///Owned chainable setter for this number is cool, returning `Self`
        #[must_use]
        pub fn with_number_in_seconds(mut self, number_in_seconds: usize) -> Self {
            self.number = number_in_seconds;
            self
        }
        ///Returns a copy of is this struct on or not
        pub fn get_enabled(&self) -> bool {
            self.enabled
        }
        ///Mutably borrow is this struct on or not
        pub fn enabled_mut(&mut self) -> &mut bool {
            &mut self.enabled
        }
        ///Sets is this struct on or not, returning `&mut Self` for chaining
        pub fn assign_enabled(&mut self, enabled: bool) -> &mut Self {
            self.enabled = enabled;
            self
        }
        ///Owned chainable setter for is this struct on or not, returning `Self`
        #[must_use]
        pub fn with_enabled(mut self, enabled: bool) -> Self {
            self.enabled = enabled;
            self
        }
        ///Borrows it's really whatever you want
        pub fn get_generic(&self) -> &T {
            &self.generic
        }
        ///Mutably borrow it's really whatever you want
        pub fn generic_mut(&mut self) -> &mut T {
            &mut self.generic
        }
        ///Sets it's really whatever you want, returning `&mut Self` for chaining
        pub fn assign_generic(&mut self, generic: T) -> &mut Self {
            self.generic = generic;
            self
        }
        ///Owned chainable setter for it's really whatever you want, returning `Self`
        #[must_use]
        pub fn with_generic(mut self, generic: T) -> Self {
            self.generic = generic;
            self
        }
    }
}
mod expand_06_docs {
    #[fieldwork(get, set, with, get_mut)]
    struct MyStruct<T> {
        /// this number is cool
        #[fieldwork(rename = number_in_seconds)]
        number: usize,
        #[fieldwork(
            get(doc = "get whether it's enabled"),
            set(doc = "assign enabled"),
            with(doc = "chainable setter for enabled"),
            get_mut(doc = "mutably borrow enabled")
        )]
        enabled: bool,
        /// it's really whatever you want
        ///
        /// Some more information about this type:
        /// - it could really be anything
        /// - we don't know much more than that
        generic: T,
    }
    impl<T> MyStruct<T> {
        ///Returns a copy of this number is cool
        pub fn number_in_seconds(&self) -> usize {
            self.number
        }
        ///Mutably borrow this number is cool
        pub fn number_in_seconds_mut(&mut self) -> &mut usize {
            &mut self.number
        }
        ///Sets this number is cool, returning `&mut Self` for chaining
        pub fn set_number_in_seconds(&mut self, number_in_seconds: usize) -> &mut Self {
            self.number = number_in_seconds;
            self
        }
        ///Owned chainable setter for this number is cool, returning `Self`
        #[must_use]
        pub fn with_number_in_seconds(mut self, number_in_seconds: usize) -> Self {
            self.number = number_in_seconds;
            self
        }
        ///get whether it's enabled
        pub fn enabled(&self) -> bool {
            self.enabled
        }
        ///mutably borrow enabled
        pub fn enabled_mut(&mut self) -> &mut bool {
            &mut self.enabled
        }
        ///assign enabled
        pub fn set_enabled(&mut self, enabled: bool) -> &mut Self {
            self.enabled = enabled;
            self
        }
        ///chainable setter for enabled
        #[must_use]
        pub fn with_enabled(mut self, enabled: bool) -> Self {
            self.enabled = enabled;
            self
        }
        /**Borrows it's really whatever you want

Some more information about this type:
- it could really be anything
- we don't know much more than that*/
        pub fn generic(&self) -> &T {
            &self.generic
        }
        /**Mutably borrow it's really whatever you want

Some more information about this type:
- it could really be anything
- we don't know much more than that*/
        pub fn generic_mut(&mut self) -> &mut T {
            &mut self.generic
        }
        /**Sets it's really whatever you want, returning `&mut Self` for chaining

Some more information about this type:
- it could really be anything
- we don't know much more than that*/
        pub fn set_generic(&mut self, generic: T) -> &mut Self {
            self.generic = generic;
            self
        }
        /**Owned chainable setter for it's really whatever you want, returning `Self`

Some more information about this type:
- it could really be anything
- we don't know much more than that*/
        #[must_use]
        pub fn with_generic(mut self, generic: T) -> Self {
            self.generic = generic;
            self
        }
    }
    #[fieldwork(
        set(doc_template = " # ssssets {}

extra info here"),
        get(doc_template = "# ggggets {}"),
        with(doc_template = "# width {}"),
        get_mut(doc_template = "# gmut {}")
    )]
    struct DocTemplates<T> {
        /// the cool number
        number: usize,
        #[fieldwork(
            get(doc = "get whether it's enabled"),
            set(doc = "assign enabled"),
            with(doc = "chainable setter for enabled"),
            get_mut(doc = "mutably borrow enabled")
        )]
        enabled: bool,
        /// the generic
        ///
        /// Some further info
        generic: T,
    }
    impl<T> DocTemplates<T> {
        ///# ggggets the cool number
        pub fn number(&self) -> usize {
            self.number
        }
        ///# gmut the cool number
        pub fn number_mut(&mut self) -> &mut usize {
            &mut self.number
        }
        /** # ssssets the cool number

extra info here*/
        pub fn set_number(&mut self, number: usize) -> &mut Self {
            self.number = number;
            self
        }
        ///# width the cool number
        #[must_use]
        pub fn with_number(mut self, number: usize) -> Self {
            self.number = number;
            self
        }
        ///get whether it's enabled
        pub fn enabled(&self) -> bool {
            self.enabled
        }
        ///mutably borrow enabled
        pub fn enabled_mut(&mut self) -> &mut bool {
            &mut self.enabled
        }
        ///assign enabled
        pub fn set_enabled(&mut self, enabled: bool) -> &mut Self {
            self.enabled = enabled;
            self
        }
        ///chainable setter for enabled
        #[must_use]
        pub fn with_enabled(mut self, enabled: bool) -> Self {
            self.enabled = enabled;
            self
        }
        /**# ggggets the generic

Some further info*/
        pub fn generic(&self) -> &T {
            &self.generic
        }
        /**# gmut the generic

Some further info*/
        pub fn generic_mut(&mut self) -> &mut T {
            &mut self.generic
        }
        /** # ssssets the generic

extra info here

Some further info*/
        pub fn set_generic(&mut self, generic: T) -> &mut Self {
            self.generic = generic;
            self
        }
        /**# width the generic

Some further info*/
        #[must_use]
        pub fn with_generic(mut self, generic: T) -> Self {
            self.generic = generic;
            self
        }
    }
}
mod expand_07_bounds {
    #[fieldwork(bounds = "T: Clone", get, set, with, get_mut)]
    struct MyStruct<T> {
        /// this number is cool
        number: usize,
        /// is this struct on or not
        enabled: bool,
        /// must be clone
        generic: T,
    }
    impl<T> MyStruct<T>
    where
        T: Clone,
    {
        ///Returns a copy of this number is cool
        pub fn number(&self) -> usize {
            self.number
        }
        ///Mutably borrow this number is cool
        pub fn number_mut(&mut self) -> &mut usize {
            &mut self.number
        }
        ///Sets this number is cool, returning `&mut Self` for chaining
        pub fn set_number(&mut self, number: usize) -> &mut Self {
            self.number = number;
            self
        }
        ///Owned chainable setter for this number is cool, returning `Self`
        #[must_use]
        pub fn with_number(mut self, number: usize) -> Self {
            self.number = number;
            self
        }
        ///Returns a copy of is this struct on or not
        pub fn enabled(&self) -> bool {
            self.enabled
        }
        ///Mutably borrow is this struct on or not
        pub fn enabled_mut(&mut self) -> &mut bool {
            &mut self.enabled
        }
        ///Sets is this struct on or not, returning `&mut Self` for chaining
        pub fn set_enabled(&mut self, enabled: bool) -> &mut Self {
            self.enabled = enabled;
            self
        }
        ///Owned chainable setter for is this struct on or not, returning `Self`
        #[must_use]
        pub fn with_enabled(mut self, enabled: bool) -> Self {
            self.enabled = enabled;
            self
        }
        ///Borrows must be clone
        pub fn generic(&self) -> &T {
            &self.generic
        }
        ///Mutably borrow must be clone
        pub fn generic_mut(&mut self) -> &mut T {
            &mut self.generic
        }
        ///Sets must be clone, returning `&mut Self` for chaining
        pub fn set_generic(&mut self, generic: T) -> &mut Self {
            self.generic = generic;
            self
        }
        ///Owned chainable setter for must be clone, returning `Self`
        #[must_use]
        pub fn with_generic(mut self, generic: T) -> Self {
            self.generic = generic;
            self
        }
    }
}
mod expand_08_arg_name {
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
    impl<T> MyStruct<T> {
        ///Returns a copy of the number
        pub fn number(&self) -> usize {
            self.number
        }
        ///Mutably borrow the number
        pub fn number_mut(&mut self) -> &mut usize {
            &mut self.number
        }
        ///Sets the number, returning `&mut Self` for chaining
        pub fn set_number(&mut self, the_number: usize) -> &mut Self {
            self.number = the_number;
            self
        }
        ///Owned chainable setter for the number, returning `Self`
        #[must_use]
        pub fn with_number(mut self, the_number: usize) -> Self {
            self.number = the_number;
            self
        }
        ///Returns a copy of whether something is enabled
        pub fn enabled(&self) -> bool {
            self.enabled
        }
        ///Mutably borrow whether something is enabled
        pub fn enabled_mut(&mut self) -> &mut bool {
            &mut self.enabled
        }
        ///Sets whether something is enabled, returning `&mut Self` for chaining
        pub fn set_enabled(&mut self, is_enabled_as_a_boolean: bool) -> &mut Self {
            self.enabled = is_enabled_as_a_boolean;
            self
        }
        ///Owned chainable setter for whether something is enabled, returning `Self`
        #[must_use]
        pub fn with_enabled(mut self, enabled: bool) -> Self {
            self.enabled = enabled;
            self
        }
        ///Borrows the generic
        pub fn generic(&self) -> &T {
            &self.generic
        }
        ///Mutably borrow the generic
        pub fn generic_mut(&mut self) -> &mut T {
            &mut self.generic
        }
        ///Sets the generic, returning `&mut Self` for chaining
        pub fn set_generic(&mut self, the_generic: T) -> &mut Self {
            self.generic = the_generic;
            self
        }
        ///Owned chainable setter for the generic, returning `Self`
        #[must_use]
        pub fn with_generic(mut self, the_gen: T) -> Self {
            self.generic = the_gen;
            self
        }
    }
}
mod expand_09_vis_override {
    #[fieldwork(vis = "pub(crate)", get, set)]
    struct MyStruct {
        /// default visibility (should be `pub(crate)`)
        number: usize,
        /// this one overrides only the getter to be fully pub
        #[fieldwork(get(vis = "pub"))]
        enabled: bool,
        /// this one overrides only the getter to be private (no `pub`)
        #[fieldwork(get(vis = ""))]
        active: bool,
        #[fieldwork(vis = "pub")]
        other: (),
        #[fieldwork(vis = "pub", get(vis = ""))]
        double_override: (),
    }
    impl MyStruct {
        ///Returns a copy of default visibility (should be `pub(crate)`)
        pub(crate) fn number(&self) -> usize {
            self.number
        }
        ///Sets default visibility (should be `pub(crate)`), returning `&mut Self` for chaining
        pub(crate) fn set_number(&mut self, number: usize) -> &mut Self {
            self.number = number;
            self
        }
        ///Returns a copy of this one overrides only the getter to be fully pub
        pub fn enabled(&self) -> bool {
            self.enabled
        }
        ///Sets this one overrides only the getter to be fully pub, returning `&mut Self` for chaining
        pub(crate) fn set_enabled(&mut self, enabled: bool) -> &mut Self {
            self.enabled = enabled;
            self
        }
        ///Returns a copy of this one overrides only the getter to be private (no `pub`)
        fn active(&self) -> bool {
            self.active
        }
        ///Sets this one overrides only the getter to be private (no `pub`), returning `&mut Self` for chaining
        pub(crate) fn set_active(&mut self, active: bool) -> &mut Self {
            self.active = active;
            self
        }
        pub fn other(&self) -> &() {
            &self.other
        }
        pub fn set_other(&mut self, other: ()) -> &mut Self {
            self.other = other;
            self
        }
        fn double_override(&self) -> &() {
            &self.double_override
        }
        pub fn set_double_override(&mut self, double_override: ()) -> &mut Self {
            self.double_override = double_override;
            self
        }
    }
    #[fieldwork(get, set)]
    struct MyStruct2 {
        /// default visibility
        number: usize,
        /// this one overrides only the getter to be fully pub
        #[fieldwork(get(vis = "pub(crate)"))]
        enabled: bool,
        /// this one overrides only the getter to be private (no `pub`)
        #[fieldwork(get(vis = ""))]
        active: bool,
        #[fieldwork(vis = "pub(crate)")]
        other: (),
        #[fieldwork(vis = "pub(crate)", get(vis = ""))]
        method_override_field: (),
    }
    impl MyStruct2 {
        ///Returns a copy of default visibility
        pub fn number(&self) -> usize {
            self.number
        }
        ///Sets default visibility, returning `&mut Self` for chaining
        pub fn set_number(&mut self, number: usize) -> &mut Self {
            self.number = number;
            self
        }
        ///Returns a copy of this one overrides only the getter to be fully pub
        pub(crate) fn enabled(&self) -> bool {
            self.enabled
        }
        ///Sets this one overrides only the getter to be fully pub, returning `&mut Self` for chaining
        pub fn set_enabled(&mut self, enabled: bool) -> &mut Self {
            self.enabled = enabled;
            self
        }
        ///Returns a copy of this one overrides only the getter to be private (no `pub`)
        fn active(&self) -> bool {
            self.active
        }
        ///Sets this one overrides only the getter to be private (no `pub`), returning `&mut Self` for chaining
        pub fn set_active(&mut self, active: bool) -> &mut Self {
            self.active = active;
            self
        }
        pub(crate) fn other(&self) -> &() {
            &self.other
        }
        pub(crate) fn set_other(&mut self, other: ()) -> &mut Self {
            self.other = other;
            self
        }
        fn method_override_field(&self) -> &() {
            &self.method_override_field
        }
        pub(crate) fn set_method_override_field(
            &mut self,
            method_override_field: (),
        ) -> &mut Self {
            self.method_override_field = method_override_field;
            self
        }
    }
}
mod expand_10_opt_in {
    #[fieldwork(opt_in, get, set, with, get_mut)]
    struct MyStruct<T> {
        /// not generated
        number: usize,
        /// generated
        #[fieldwork]
        enabled: bool,
        /// generated
        #[fieldwork]
        generic: T,
        #[fieldwork(get)]
        only_get: (),
    }
    impl<T> MyStruct<T> {
        ///Returns a copy of generated
        pub fn enabled(&self) -> bool {
            self.enabled
        }
        ///Mutably borrow generated
        pub fn enabled_mut(&mut self) -> &mut bool {
            &mut self.enabled
        }
        ///Sets generated, returning `&mut Self` for chaining
        pub fn set_enabled(&mut self, enabled: bool) -> &mut Self {
            self.enabled = enabled;
            self
        }
        ///Owned chainable setter for generated, returning `Self`
        #[must_use]
        pub fn with_enabled(mut self, enabled: bool) -> Self {
            self.enabled = enabled;
            self
        }
        ///Borrows generated
        pub fn generic(&self) -> &T {
            &self.generic
        }
        ///Mutably borrow generated
        pub fn generic_mut(&mut self) -> &mut T {
            &mut self.generic
        }
        ///Sets generated, returning `&mut Self` for chaining
        pub fn set_generic(&mut self, generic: T) -> &mut Self {
            self.generic = generic;
            self
        }
        ///Owned chainable setter for generated, returning `Self`
        #[must_use]
        pub fn with_generic(mut self, generic: T) -> Self {
            self.generic = generic;
            self
        }
        pub fn only_get(&self) -> &() {
            &self.only_get
        }
    }
    #[fieldwork(opt_in, get, set, with, get_mut)]
    struct FieldworkEqualsTrue<T> {
        /// not generated
        number: usize,
        /// generated
        #[fieldwork = true]
        enabled: bool,
        /// generated
        #[field = true]
        generic: T,
    }
    impl<T> FieldworkEqualsTrue<T> {
        ///Returns a copy of generated
        pub fn enabled(&self) -> bool {
            self.enabled
        }
        ///Mutably borrow generated
        pub fn enabled_mut(&mut self) -> &mut bool {
            &mut self.enabled
        }
        ///Sets generated, returning `&mut Self` for chaining
        pub fn set_enabled(&mut self, enabled: bool) -> &mut Self {
            self.enabled = enabled;
            self
        }
        ///Owned chainable setter for generated, returning `Self`
        #[must_use]
        pub fn with_enabled(mut self, enabled: bool) -> Self {
            self.enabled = enabled;
            self
        }
        ///Borrows generated
        pub fn generic(&self) -> &T {
            &self.generic
        }
        ///Mutably borrow generated
        pub fn generic_mut(&mut self) -> &mut T {
            &mut self.generic
        }
        ///Sets generated, returning `&mut Self` for chaining
        pub fn set_generic(&mut self, generic: T) -> &mut Self {
            self.generic = generic;
            self
        }
        ///Owned chainable setter for generated, returning `Self`
        #[must_use]
        pub fn with_generic(mut self, generic: T) -> Self {
            self.generic = generic;
            self
        }
    }
    #[fieldwork(opt_in, get)]
    struct OptingInPerField<T> {
        /// not generated
        number: usize,
        /// generated
        #[fieldwork(set = true, get(skip))]
        enabled: bool,
        /// generated
        #[fieldwork]
        generic: T,
    }
    impl<T> OptingInPerField<T> {
        ///Sets generated, returning `&mut Self` for chaining
        pub fn set_enabled(&mut self, enabled: bool) -> &mut Self {
            self.enabled = enabled;
            self
        }
        ///Borrows generated
        pub fn generic(&self) -> &T {
            &self.generic
        }
    }
    #[fieldwork(opt_in, get(template = "get_{}"))]
    struct OptingInPerField2<T> {
        /// not generated
        number: usize,
        /// generated
        #[fieldwork(set)]
        enabled: bool,
        /// generated
        #[fieldwork(get, set = true)]
        generic: T,
    }
    impl<T> OptingInPerField2<T> {
        ///Sets generated, returning `&mut Self` for chaining
        pub fn set_enabled(&mut self, enabled: bool) -> &mut Self {
            self.enabled = enabled;
            self
        }
        ///Borrows generated
        pub fn get_generic(&self) -> &T {
            &self.generic
        }
        ///Sets generated, returning `&mut Self` for chaining
        pub fn set_generic(&mut self, generic: T) -> &mut Self {
            self.generic = generic;
            self
        }
    }
    #[fieldwork(get, set, with, get_mut)]
    struct MyStruct2 {
        number: usize,
        #[fieldwork(opt_in = true, get)]
        only_get: (),
    }
    impl MyStruct2 {
        pub fn number(&self) -> usize {
            self.number
        }
        pub fn number_mut(&mut self) -> &mut usize {
            &mut self.number
        }
        pub fn set_number(&mut self, number: usize) -> &mut Self {
            self.number = number;
            self
        }
        #[must_use]
        pub fn with_number(mut self, number: usize) -> Self {
            self.number = number;
            self
        }
        pub fn only_get(&self) -> &() {
            &self.only_get
        }
    }
}
mod expand_11_copy {
    #[fieldwork(get)]
    struct MyStruct<T: Copy> {
        number: usize,
        enabled: bool,
        #[fieldwork(get(copy))]
        generic: T,
        #[fieldwork(get(copy = true))]
        another: usize,
        static_str: &'static str,
    }
    impl<T: Copy> MyStruct<T> {
        pub fn number(&self) -> usize {
            self.number
        }
        pub fn enabled(&self) -> bool {
            self.enabled
        }
        pub fn generic(&self) -> T {
            self.generic
        }
        pub fn another(&self) -> usize {
            self.another
        }
        pub fn static_str(&self) -> &'static str {
            self.static_str
        }
    }
    #[fieldwork(get)]
    struct HoldsAReference<'a> {
        name: &'a str,
        mut_ref_not_copy: &'a mut (),
    }
    impl<'a> HoldsAReference<'a> {
        pub fn name(&self) -> &'a str {
            self.name
        }
        pub fn mut_ref_not_copy(&self) -> &() {
            &*self.mut_ref_not_copy
        }
    }
    #[fieldwork(get)]
    struct AllCopyTypes {
        char: char,
        f32: f32,
        f64: f64,
        i128: i128,
        i16: i16,
        i32: i32,
        i8: i8,
        isize: isize,
        u128: u128,
        u16: u16,
        u32: u32,
        u8: u8,
        usize: usize,
        bool: bool,
    }
    impl AllCopyTypes {
        pub fn char(&self) -> char {
            self.char
        }
        pub fn f32(&self) -> f32 {
            self.f32
        }
        pub fn f64(&self) -> f64 {
            self.f64
        }
        pub fn i128(&self) -> i128 {
            self.i128
        }
        pub fn i16(&self) -> i16 {
            self.i16
        }
        pub fn i32(&self) -> i32 {
            self.i32
        }
        pub fn i8(&self) -> i8 {
            self.i8
        }
        pub fn isize(&self) -> isize {
            self.isize
        }
        pub fn u128(&self) -> u128 {
            self.u128
        }
        pub fn u16(&self) -> u16 {
            self.u16
        }
        pub fn u32(&self) -> u32 {
            self.u32
        }
        pub fn u8(&self) -> u8 {
            self.u8
        }
        pub fn usize(&self) -> usize {
            self.usize
        }
        pub fn bool(&self) -> bool {
            self.bool
        }
    }
    mod x {
        use std::sync::Arc;
        #[fieldwork(get)]
        struct CopyInteractsWithDeref {
            box_bool: Box<bool>,
            arc_usize: Arc<usize>,
        }
        impl CopyInteractsWithDeref {
            pub fn box_bool(&self) -> bool {
                *self.box_bool
            }
            pub fn arc_usize(&self) -> usize {
                *self.arc_usize
            }
        }
    }
    #[fieldwork(get)]
    struct AllowSpecifyingCopyAtFieldAttribute<T: Copy> {
        #[fieldwork(copy)]
        generic: T,
        #[fieldwork(copy = true)]
        another: (T, T),
    }
    impl<T: Copy> AllowSpecifyingCopyAtFieldAttribute<T> {
        pub fn generic(&self) -> T {
            self.generic
        }
        pub fn another(&self) -> (T, T) {
            self.another
        }
    }
    #[fieldwork(get)]
    struct AllowSpecifyingCopyFalseAtFieldAttribute {
        #[fieldwork(copy = false)]
        usize: usize,
    }
    impl AllowSpecifyingCopyFalseAtFieldAttribute {
        pub fn usize(&self) -> &usize {
            &self.usize
        }
    }
    #[fieldwork(get, copy = false)]
    struct AllowOptingBackInAtFieldAttribute<T: Copy> {
        #[fieldwork(copy = true)]
        usize: usize,
        #[field(copy)]
        generic: T,
    }
    impl<T: Copy> AllowOptingBackInAtFieldAttribute<T> {
        pub fn usize(&self) -> usize {
            self.usize
        }
        pub fn generic(&self) -> T {
            self.generic
        }
    }
}
mod expand_12_non_chain_set {
    #[fieldwork(set)]
    struct MyStruct<T> {
        number: usize,
        /// opting out
        #[fieldwork(set(chain = false))]
        enabled: bool,
        generic: T,
    }
    impl<T> MyStruct<T> {
        pub fn set_number(&mut self, number: usize) -> &mut Self {
            self.number = number;
            self
        }
        ///Sets opting out
        pub fn set_enabled(&mut self, enabled: bool) {
            self.enabled = enabled;
        }
        pub fn set_generic(&mut self, generic: T) -> &mut Self {
            self.generic = generic;
            self
        }
    }
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
    impl<T> MyStruct2<T> {
        ///Sets opted out at struct-method level
        pub fn set_number(&mut self, number: usize) {
            self.number = number;
        }
        ///Sets opting back in, returning `&mut Self` for chaining
        pub fn set_enabled(&mut self, enabled: bool) -> &mut Self {
            self.enabled = enabled;
            self
        }
        ///Sets opting back in, returning `&mut Self` for chaining
        pub fn set_generic(&mut self, generic: T) -> &mut Self {
            self.generic = generic;
            self
        }
    }
}
mod expand_13_deref {
    struct DerefType;
    struct OwnedType(DerefType);
    impl std::ops::Deref for OwnedType {
        type Target = DerefType;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl std::ops::DerefMut for OwnedType {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    struct User {
        #[fieldwork(deref = DerefType, get, set, get_mut)]
        deref_both: OwnedType,
    }
    impl User {
        pub fn deref_both(&self) -> &DerefType {
            &*self.deref_both
        }
        pub fn deref_both_mut(&mut self) -> &mut DerefType {
            &mut *self.deref_both
        }
        pub fn set_deref_both(&mut self, deref_both: OwnedType) -> &mut Self {
            self.deref_both = deref_both;
            self
        }
    }
    struct OnlyDerefForMethods {
        #[fieldwork(get(deref = DerefType), set, get_mut)]
        deref_for_get_only: OwnedType,
        #[fieldwork(get, set, get_mut(deref = "&DerefType"))]
        deref_for_get_mut_only: OwnedType,
    }
    impl OnlyDerefForMethods {
        pub fn deref_for_get_only(&self) -> &DerefType {
            &*self.deref_for_get_only
        }
        pub fn deref_for_get_only_mut(&mut self) -> &mut OwnedType {
            &mut self.deref_for_get_only
        }
        pub fn set_deref_for_get_only(
            &mut self,
            deref_for_get_only: OwnedType,
        ) -> &mut Self {
            self.deref_for_get_only = deref_for_get_only;
            self
        }
        pub fn deref_for_get_mut_only(&self) -> &OwnedType {
            &self.deref_for_get_mut_only
        }
        pub fn deref_for_get_mut_only_mut(&mut self) -> &mut DerefType {
            &mut *self.deref_for_get_mut_only
        }
        pub fn set_deref_for_get_mut_only(
            &mut self,
            deref_for_get_mut_only: OwnedType,
        ) -> &mut Self {
            self.deref_for_get_mut_only = deref_for_get_mut_only;
            self
        }
    }
}
mod expand_14_option_detection {
    struct DerefType;
    struct OwnedType(DerefType);
    impl std::ops::Deref for OwnedType {
        type Target = DerefType;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl std::ops::DerefMut for OwnedType {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    #[fieldwork(get, get_mut)]
    struct OptionBehavior {
        #[fieldwork(deref = "Option<&str>")]
        option_deref: Option<String>,
        #[fieldwork(deref = str)]
        option_deref_other_style: Option<String>,
        #[fieldwork(option_borrow_inner = false)]
        no_option_detection: Option<String>,
        option_detection: Option<()>,
        #[fieldwork(get_mut(option_borrow_inner = false))]
        nothing_fancy_for_get_mut: Option<()>,
    }
    impl OptionBehavior {
        pub fn option_deref(&self) -> Option<&str> {
            self.option_deref.as_deref()
        }
        pub fn option_deref_mut(&mut self) -> Option<&mut str> {
            self.option_deref.as_deref_mut()
        }
        pub fn option_deref_other_style(&self) -> Option<&str> {
            self.option_deref_other_style.as_deref()
        }
        pub fn option_deref_other_style_mut(&mut self) -> Option<&mut str> {
            self.option_deref_other_style.as_deref_mut()
        }
        pub fn no_option_detection(&self) -> &Option<String> {
            &self.no_option_detection
        }
        pub fn no_option_detection_mut(&mut self) -> &mut Option<String> {
            &mut self.no_option_detection
        }
        pub fn option_detection(&self) -> Option<&()> {
            self.option_detection.as_ref()
        }
        pub fn option_detection_mut(&mut self) -> Option<&mut ()> {
            self.option_detection.as_mut()
        }
        pub fn nothing_fancy_for_get_mut(&self) -> Option<&()> {
            self.nothing_fancy_for_get_mut.as_ref()
        }
        pub fn nothing_fancy_for_get_mut_mut(&mut self) -> &mut Option<()> {
            &mut self.nothing_fancy_for_get_mut
        }
    }
    #[fieldwork(get, get_mut, option_borrow_inner = false)]
    struct OptInOption {
        #[fieldwork(deref = "Option<&str>", option_borrow_inner = true)]
        option_deref: Option<String>,
        #[fieldwork(deref = str, option_borrow_inner = true)]
        option_deref_other_style: Option<String>,
        #[fieldwork(option_borrow_inner)]
        option_detection: Option<String>,
        no_option_detection: Option<()>,
        #[fieldwork(get_mut(option_borrow_inner = true))]
        option_detection_only_get_mut: Option<()>,
    }
    impl OptInOption {
        pub fn option_deref(&self) -> Option<&str> {
            self.option_deref.as_deref()
        }
        pub fn option_deref_mut(&mut self) -> Option<&mut str> {
            self.option_deref.as_deref_mut()
        }
        pub fn option_deref_other_style(&self) -> Option<&str> {
            self.option_deref_other_style.as_deref()
        }
        pub fn option_deref_other_style_mut(&mut self) -> Option<&mut str> {
            self.option_deref_other_style.as_deref_mut()
        }
        pub fn option_detection(&self) -> Option<&str> {
            self.option_detection.as_deref()
        }
        pub fn option_detection_mut(&mut self) -> Option<&mut str> {
            self.option_detection.as_deref_mut()
        }
        pub fn no_option_detection(&self) -> &Option<()> {
            &self.no_option_detection
        }
        pub fn no_option_detection_mut(&mut self) -> &mut Option<()> {
            &mut self.no_option_detection
        }
        pub fn option_detection_only_get_mut(&self) -> &Option<()> {
            &self.option_detection_only_get_mut
        }
        pub fn option_detection_only_get_mut_mut(&mut self) -> Option<&mut ()> {
            self.option_detection_only_get_mut.as_mut()
        }
    }
    #[fieldwork(get(option_borrow_inner = true), get_mut, option_borrow_inner = false)]
    struct OptionOnlyForGet {
        #[fieldwork(get(option_borrow_inner = false))]
        no_option_detection: Option<()>,
        #[fieldwork(get(deref = str))]
        option_deref: Option<String>,
        #[fieldwork(option_borrow_inner = true)]
        field_overrides: Option<String>,
        option_only_for_get: Option<()>,
        #[fieldwork(get_mut(option_borrow_inner), get(option_borrow_inner = false))]
        option_detection_only_get_mut: Option<()>,
    }
    impl OptionOnlyForGet {
        pub fn no_option_detection(&self) -> &Option<()> {
            &self.no_option_detection
        }
        pub fn no_option_detection_mut(&mut self) -> &mut Option<()> {
            &mut self.no_option_detection
        }
        pub fn option_deref(&self) -> Option<&str> {
            self.option_deref.as_deref()
        }
        pub fn option_deref_mut(&mut self) -> &mut Option<String> {
            &mut self.option_deref
        }
        pub fn field_overrides(&self) -> Option<&str> {
            self.field_overrides.as_deref()
        }
        pub fn field_overrides_mut(&mut self) -> Option<&mut str> {
            self.field_overrides.as_deref_mut()
        }
        pub fn option_only_for_get(&self) -> Option<&()> {
            self.option_only_for_get.as_ref()
        }
        pub fn option_only_for_get_mut(&mut self) -> &mut Option<()> {
            &mut self.option_only_for_get
        }
        pub fn option_detection_only_get_mut(&self) -> &Option<()> {
            &self.option_detection_only_get_mut
        }
        pub fn option_detection_only_get_mut_mut(&mut self) -> Option<&mut ()> {
            self.option_detection_only_get_mut.as_mut()
        }
    }
    #[fieldwork(get, get_mut, option_borrow_inner = false, deref = false)]
    struct OptionAndDerefInteraction {
        a: Option<String>,
        #[fieldwork(option_borrow_inner)]
        b: Option<String>,
        #[fieldwork(option_borrow_inner, deref)]
        c: Option<String>,
        #[fieldwork(deref)]
        d: Option<String>,
        #[fieldwork(option_borrow_inner, deref = "Option<&DerefType>")]
        e: Option<OwnedType>,
    }
    impl OptionAndDerefInteraction {
        pub fn a(&self) -> &Option<String> {
            &self.a
        }
        pub fn a_mut(&mut self) -> &mut Option<String> {
            &mut self.a
        }
        pub fn b(&self) -> Option<&String> {
            self.b.as_ref()
        }
        pub fn b_mut(&mut self) -> Option<&mut String> {
            self.b.as_mut()
        }
        pub fn c(&self) -> Option<&str> {
            self.c.as_deref()
        }
        pub fn c_mut(&mut self) -> Option<&mut str> {
            self.c.as_deref_mut()
        }
        pub fn d(&self) -> &Option<String> {
            &self.d
        }
        pub fn d_mut(&mut self) -> &mut Option<String> {
            &mut self.d
        }
        pub fn e(&self) -> Option<&DerefType> {
            self.e.as_deref()
        }
        pub fn e_mut(&mut self) -> Option<&mut DerefType> {
            self.e.as_deref_mut()
        }
    }
    #[fieldwork(get, option = false)]
    struct BackwardsCompat {
        #[fieldwork(option)]
        borrow_inner: Option<()>,
        not_borrow_inner: Option<String>,
    }
    impl BackwardsCompat {
        pub fn borrow_inner(&self) -> Option<&()> {
            self.borrow_inner.as_ref()
        }
        pub fn not_borrow_inner(&self) -> &Option<String> {
            &self.not_borrow_inner
        }
    }
}
mod expand_15_auto_deref {
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
        pub fn baseline_deref_for_get_but_not_for_get_mut_mut(
            &mut self,
        ) -> &mut Vec<()> {
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
}
mod expand_17_option_set_some {
    #[fieldwork(set, with, get, get_mut, option_set_some)]
    struct OptionBehavior {
        #[fieldwork(deref = "Option<&str>")]
        option_deref: Option<String>,
        #[fieldwork(option_set_some = false)]
        no_option_detection: Option<bool>,
        option_detection: Option<()>,
        #[fieldwork(set(option_set_some = false, option_borrow_inner = false))]
        nothing_fancy_for_set: Option<String>,
    }
    impl OptionBehavior {
        pub fn option_deref(&self) -> Option<&str> {
            self.option_deref.as_deref()
        }
        pub fn option_deref_mut(&mut self) -> Option<&mut str> {
            self.option_deref.as_deref_mut()
        }
        pub fn set_option_deref(&mut self, option_deref: String) -> &mut Self {
            self.option_deref = Some(option_deref);
            self
        }
        #[must_use]
        pub fn with_option_deref(mut self, option_deref: String) -> Self {
            self.option_deref = Some(option_deref);
            self
        }
        pub fn no_option_detection(&self) -> Option<bool> {
            self.no_option_detection
        }
        pub fn no_option_detection_mut(&mut self) -> Option<&mut bool> {
            self.no_option_detection.as_mut()
        }
        pub fn set_no_option_detection(
            &mut self,
            no_option_detection: Option<bool>,
        ) -> &mut Self {
            self.no_option_detection = no_option_detection;
            self
        }
        #[must_use]
        pub fn with_no_option_detection(
            mut self,
            no_option_detection: Option<bool>,
        ) -> Self {
            self.no_option_detection = no_option_detection;
            self
        }
        pub fn option_detection(&self) -> Option<&()> {
            self.option_detection.as_ref()
        }
        pub fn option_detection_mut(&mut self) -> Option<&mut ()> {
            self.option_detection.as_mut()
        }
        pub fn set_option_detection(&mut self, option_detection: ()) -> &mut Self {
            self.option_detection = Some(option_detection);
            self
        }
        #[must_use]
        pub fn with_option_detection(mut self, option_detection: ()) -> Self {
            self.option_detection = Some(option_detection);
            self
        }
        pub fn nothing_fancy_for_set(&self) -> Option<&str> {
            self.nothing_fancy_for_set.as_deref()
        }
        pub fn nothing_fancy_for_set_mut(&mut self) -> Option<&mut str> {
            self.nothing_fancy_for_set.as_deref_mut()
        }
        pub fn set_nothing_fancy_for_set(
            &mut self,
            nothing_fancy_for_set: Option<String>,
        ) -> &mut Self {
            self.nothing_fancy_for_set = nothing_fancy_for_set;
            self
        }
        #[must_use]
        pub fn with_nothing_fancy_for_set(
            mut self,
            nothing_fancy_for_set: String,
        ) -> Self {
            self.nothing_fancy_for_set = Some(nothing_fancy_for_set);
            self
        }
    }
    #[fieldwork(with(option_set_some))]
    struct BobTheBuilder {
        string: Option<String>,
        bool: Option<bool>,
    }
    impl BobTheBuilder {
        #[must_use]
        pub fn with_string(mut self, string: String) -> Self {
            self.string = Some(string);
            self
        }
        #[must_use]
        pub fn with_bool(mut self, bool: bool) -> Self {
            self.bool = Some(bool);
            self
        }
    }
    #[fieldwork(set, with, get, get_mut, option_set_some)]
    struct HandlesNonOptionTypes {
        string: String,
        bool: bool,
    }
    impl HandlesNonOptionTypes {
        pub fn string(&self) -> &str {
            &*self.string
        }
        pub fn string_mut(&mut self) -> &mut str {
            &mut *self.string
        }
        pub fn set_string(&mut self, string: String) -> &mut Self {
            self.string = string;
            self
        }
        #[must_use]
        pub fn with_string(mut self, string: String) -> Self {
            self.string = string;
            self
        }
        pub fn bool(&self) -> bool {
            self.bool
        }
        pub fn bool_mut(&mut self) -> &mut bool {
            &mut self.bool
        }
        pub fn set_bool(&mut self, bool: bool) -> &mut Self {
            self.bool = bool;
            self
        }
        #[must_use]
        pub fn with_bool(mut self, bool: bool) -> Self {
            self.bool = bool;
            self
        }
    }
}
mod expand_18_into {
    #[fieldwork(get, set, get_mut, with)]
    struct AcceptsAnythingInto {
        #[fieldwork(into)]
        string: String,
        #[fieldwork(option_set_some, into)]
        option_string: Option<String>,
    }
    impl AcceptsAnythingInto {
        pub fn string(&self) -> &str {
            &*self.string
        }
        pub fn string_mut(&mut self) -> &mut str {
            &mut *self.string
        }
        pub fn set_string(&mut self, string: impl Into<String>) -> &mut Self {
            self.string = string.into();
            self
        }
        #[must_use]
        pub fn with_string(mut self, string: impl Into<String>) -> Self {
            self.string = string.into();
            self
        }
        pub fn option_string(&self) -> Option<&str> {
            self.option_string.as_deref()
        }
        pub fn option_string_mut(&mut self) -> Option<&mut str> {
            self.option_string.as_deref_mut()
        }
        pub fn set_option_string(
            &mut self,
            option_string: impl Into<String>,
        ) -> &mut Self {
            self.option_string = Some(option_string.into());
            self
        }
        #[must_use]
        pub fn with_option_string(mut self, option_string: impl Into<String>) -> Self {
            self.option_string = Some(option_string.into());
            self
        }
    }
}
mod expand_19_tuple_structs {
    use std::borrow::Cow;
    use std::ops::{Deref, DerefMut};
    use std::path::PathBuf;
    use std::sync::Arc;
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
    #[automatically_derived]
    impl ::core::clone::Clone for Rgb {
        #[inline]
        fn clone(&self) -> Rgb {
            let _: ::core::clone::AssertParamIsClone<u8>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Rgb {}
    #[fieldwork(get, set, with, get_mut, option_set_some)]
    struct Color(
        #[fieldwork(name = rgb, copy)]
        Rgb,
        #[fieldwork(name = alpha)]
        Option<u8>,
    );
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
        pub fn a(&self) -> Option<&std::path::Path> {
            self.0.as_ref().map(|a| &***a)
        }
        pub fn a_mut(&mut self) -> Option<&mut std::rc::Rc<PathBuf>> {
            self.0.as_mut()
        }
        pub fn b(&self) -> Option<&T> {
            self.1.as_ref().map(|b| &****b)
        }
        pub fn b_mut(&mut self) -> Option<&mut Arc<Cow<'a, T>>> {
            self.1.as_deref_mut()
        }
        pub fn c(&self) -> Option<&[u8]> {
            self.2.as_ref().map(|c| &***c)
        }
        pub fn c_mut(&mut self) -> Option<&mut Arc<Vec<u8>>> {
            self.2.as_mut()
        }
        pub fn d(&self) -> Option<&[T]> {
            self.3.as_ref().map(|d| &***d)
        }
        pub fn d_mut(&mut self) -> Option<&mut [T]> {
            self.3.as_mut().map(|d| &mut ***d)
        }
        pub fn e(&self) -> Option<&T::Target> {
            self.4.as_ref().map(|e| &***e)
        }
        pub fn e_mut(&mut self) -> Option<&mut T::Target> {
            self.4.as_mut().map(|e| &mut ***e)
        }
    }
}
mod expand_20_with_and_without {
    use std::path::PathBuf;
    #[fieldwork(with, without)]
    struct WithAndWithout {
        tls: bool,
        debug_output: bool,
        name: Option<String>,
        #[fieldwork(without = false)]
        without_skipped: Option<usize>,
        #[fieldwork(without = false)]
        bool_no_without: bool,
        #[fieldwork(option_set_some = false)]
        no_option_set_some: Option<u8>,
    }
    impl WithAndWithout {
        #[must_use]
        pub fn with_tls(mut self) -> Self {
            self.tls = true;
            self
        }
        #[must_use]
        pub fn without_tls(mut self) -> Self {
            self.tls = false;
            self
        }
        #[must_use]
        pub fn with_debug_output(mut self) -> Self {
            self.debug_output = true;
            self
        }
        #[must_use]
        pub fn without_debug_output(mut self) -> Self {
            self.debug_output = false;
            self
        }
        #[must_use]
        pub fn with_name(mut self, name: String) -> Self {
            self.name = Some(name);
            self
        }
        #[must_use]
        pub fn without_name(mut self) -> Self {
            self.name = None;
            self
        }
        #[must_use]
        pub fn with_without_skipped(mut self, without_skipped: Option<usize>) -> Self {
            self.without_skipped = without_skipped;
            self
        }
        #[must_use]
        pub fn with_bool_no_without(mut self, bool_no_without: bool) -> Self {
            self.bool_no_without = bool_no_without;
            self
        }
        #[must_use]
        pub fn with_no_option_set_some(
            mut self,
            no_option_set_some: Option<u8>,
        ) -> Self {
            self.no_option_set_some = no_option_set_some;
            self
        }
        #[must_use]
        pub fn without_no_option_set_some(mut self) -> Self {
            self.no_option_set_some = None;
            self
        }
    }
    #[fieldwork(with, without)]
    struct MoreEdgeCases {
        #[fieldwork(into)]
        path_with_into: Option<PathBuf>,
        #[fieldwork(rename = custom_name)]
        renamed_field: Option<String>,
        #[fieldwork(with(rename = special_with), without(rename = special_clear))]
        custom_method_names: Option<i32>,
        nested_option: Option<Option<String>>,
        option_vec: Option<Vec<u8>>,
        option_result: Option<Result<String, ()>>,
        #[fieldwork(vis = "pub(crate)")]
        limited_visibility: Option<bool>,
        #[fieldwork(with = false)]
        without_only: Option<String>,
        #[fieldwork(argument = value)]
        custom_arg_name: Option<u64>,
        deref_option: Option<Vec<String>>,
        #[fieldwork(skip)]
        completely_skipped: Option<bool>,
        #[fieldwork(with(skip))]
        with_skipped_without_allowed: Option<String>,
    }
    impl MoreEdgeCases {
        #[must_use]
        pub fn with_path_with_into(
            mut self,
            path_with_into: impl Into<PathBuf>,
        ) -> Self {
            self.path_with_into = Some(path_with_into.into());
            self
        }
        #[must_use]
        pub fn without_path_with_into(mut self) -> Self {
            self.path_with_into = None;
            self
        }
        #[must_use]
        pub fn with_custom_name(mut self, custom_name: String) -> Self {
            self.renamed_field = Some(custom_name);
            self
        }
        #[must_use]
        pub fn without_custom_name(mut self) -> Self {
            self.renamed_field = None;
            self
        }
        #[must_use]
        pub fn special_with(mut self, custom_method_names: i32) -> Self {
            self.custom_method_names = Some(custom_method_names);
            self
        }
        #[must_use]
        pub fn special_clear(mut self) -> Self {
            self.custom_method_names = None;
            self
        }
        #[must_use]
        pub fn with_nested_option(mut self, nested_option: Option<String>) -> Self {
            self.nested_option = Some(nested_option);
            self
        }
        #[must_use]
        pub fn without_nested_option(mut self) -> Self {
            self.nested_option = None;
            self
        }
        #[must_use]
        pub fn with_option_vec(mut self, option_vec: Vec<u8>) -> Self {
            self.option_vec = Some(option_vec);
            self
        }
        #[must_use]
        pub fn without_option_vec(mut self) -> Self {
            self.option_vec = None;
            self
        }
        #[must_use]
        pub fn with_option_result(mut self, option_result: Result<String, ()>) -> Self {
            self.option_result = Some(option_result);
            self
        }
        #[must_use]
        pub fn without_option_result(mut self) -> Self {
            self.option_result = None;
            self
        }
        #[must_use]
        pub(crate) fn with_limited_visibility(
            mut self,
            limited_visibility: bool,
        ) -> Self {
            self.limited_visibility = Some(limited_visibility);
            self
        }
        #[must_use]
        pub(crate) fn without_limited_visibility(mut self) -> Self {
            self.limited_visibility = None;
            self
        }
        #[must_use]
        pub fn without_without_only(mut self) -> Self {
            self.without_only = None;
            self
        }
        #[must_use]
        pub fn with_custom_arg_name(mut self, value: u64) -> Self {
            self.custom_arg_name = Some(value);
            self
        }
        #[must_use]
        pub fn without_custom_arg_name(mut self) -> Self {
            self.custom_arg_name = None;
            self
        }
        #[must_use]
        pub fn with_deref_option(mut self, deref_option: Vec<String>) -> Self {
            self.deref_option = Some(deref_option);
            self
        }
        #[must_use]
        pub fn without_deref_option(mut self) -> Self {
            self.deref_option = None;
            self
        }
        #[must_use]
        pub fn without_with_skipped_without_allowed(mut self) -> Self {
            self.with_skipped_without_allowed = None;
            self
        }
    }
    #[fieldwork(without)]
    struct WithoutIsSkipped {
        string_field: String,
        number_field: u32,
    }
    impl WithoutIsSkipped {}
    #[fieldwork(with, without)]
    struct Mixed {
        name: Option<String>,
        active: bool,
        count: u32,
        data: String,
    }
    impl Mixed {
        #[must_use]
        pub fn with_name(mut self, name: String) -> Self {
            self.name = Some(name);
            self
        }
        #[must_use]
        pub fn without_name(mut self) -> Self {
            self.name = None;
            self
        }
        #[must_use]
        pub fn with_active(mut self) -> Self {
            self.active = true;
            self
        }
        #[must_use]
        pub fn without_active(mut self) -> Self {
            self.active = false;
            self
        }
        #[must_use]
        pub fn with_count(mut self, count: u32) -> Self {
            self.count = count;
            self
        }
        #[must_use]
        pub fn with_data(mut self, data: String) -> Self {
            self.data = data;
            self
        }
    }
}
mod expand_21_lifetimes {
    use std::borrow::Cow;
    #[fieldwork(get, get_mut)]
    struct MyStruct<'a> {
        borrow: &'a (),
        mut_borrow: &'a mut (),
        cow: Cow<'a, str>,
        box_dyn_trait: Box<dyn std::fmt::Debug + 'a>,
    }
    impl<'a> MyStruct<'a> {
        pub fn borrow(&self) -> &'a () {
            self.borrow
        }
        pub fn borrow_mut(&mut self) -> &mut &'a () {
            &mut self.borrow
        }
        pub fn mut_borrow(&self) -> &() {
            &*self.mut_borrow
        }
        pub fn mut_borrow_mut(&mut self) -> &mut () {
            &mut *self.mut_borrow
        }
        pub fn cow(&self) -> &str {
            &*self.cow
        }
        pub fn cow_mut(&mut self) -> &mut Cow<'a, str> {
            &mut self.cow
        }
        pub fn box_dyn_trait(&self) -> &(dyn std::fmt::Debug + 'a) {
            &*self.box_dyn_trait
        }
        pub fn box_dyn_trait_mut(&mut self) -> &mut (dyn std::fmt::Debug + 'a) {
            &mut *self.box_dyn_trait
        }
    }
}
mod expand_22_arrays {
    use std::sync::Arc;
    #[fieldwork(get, get_mut)]
    struct DebugStruct {
        array: [u8; 10],
        box_array: Box<[u8; 10]>,
        arc_box_array: Arc<Box<[u8; 10]>>,
        option_array: Option<[u8; 10]>,
        option_box_array: Option<Box<[u8; 10]>>,
        option_arc_box_array: Option<Arc<Box<[u8; 10]>>>,
    }
    impl DebugStruct {
        pub fn array(&self) -> &[u8] {
            &self.array[..]
        }
        pub fn array_mut(&mut self) -> &mut [u8] {
            &mut self.array[..]
        }
        pub fn box_array(&self) -> &[u8] {
            &self.box_array[..]
        }
        pub fn box_array_mut(&mut self) -> &mut [u8] {
            &mut self.box_array[..]
        }
        pub fn arc_box_array(&self) -> &[u8] {
            &self.arc_box_array[..]
        }
        pub fn arc_box_array_mut(&mut self) -> &mut Arc<Box<[u8; 10]>> {
            &mut self.arc_box_array
        }
        pub fn option_array(&self) -> Option<&[u8]> {
            self.option_array.as_ref().map(|option_array| &option_array[..])
        }
        pub fn option_array_mut(&mut self) -> Option<&mut [u8]> {
            self.option_array.as_mut().map(|option_array| &mut option_array[..])
        }
        pub fn option_box_array(&self) -> Option<&[u8]> {
            self.option_box_array.as_ref().map(|option_box_array| &option_box_array[..])
        }
        pub fn option_box_array_mut(&mut self) -> Option<&mut [u8]> {
            self.option_box_array
                .as_mut()
                .map(|option_box_array| &mut option_box_array[..])
        }
        pub fn option_arc_box_array(&self) -> Option<&[u8]> {
            self.option_arc_box_array
                .as_ref()
                .map(|option_arc_box_array| &option_arc_box_array[..])
        }
        pub fn option_arc_box_array_mut(&mut self) -> Option<&mut Arc<Box<[u8; 10]>>> {
            self.option_arc_box_array.as_mut()
        }
    }
}
