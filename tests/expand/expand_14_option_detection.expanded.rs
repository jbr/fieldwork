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
    #[inline]
    pub fn option_deref(&self) -> Option<&str> {
        self.option_deref.as_deref()
    }
    #[inline]
    pub fn option_deref_mut(&mut self) -> Option<&mut str> {
        self.option_deref.as_deref_mut()
    }
    #[inline]
    pub fn option_deref_other_style(&self) -> Option<&str> {
        self.option_deref_other_style.as_deref()
    }
    #[inline]
    pub fn option_deref_other_style_mut(&mut self) -> Option<&mut str> {
        self.option_deref_other_style.as_deref_mut()
    }
    #[inline]
    pub fn no_option_detection(&self) -> &Option<String> {
        &self.no_option_detection
    }
    #[inline]
    pub fn no_option_detection_mut(&mut self) -> &mut Option<String> {
        &mut self.no_option_detection
    }
    #[inline]
    pub fn option_detection(&self) -> Option<&()> {
        self.option_detection.as_ref()
    }
    #[inline]
    pub fn option_detection_mut(&mut self) -> Option<&mut ()> {
        self.option_detection.as_mut()
    }
    #[inline]
    pub fn nothing_fancy_for_get_mut(&self) -> Option<&()> {
        self.nothing_fancy_for_get_mut.as_ref()
    }
    #[inline]
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
    #[inline]
    pub fn option_deref(&self) -> Option<&str> {
        self.option_deref.as_deref()
    }
    #[inline]
    pub fn option_deref_mut(&mut self) -> Option<&mut str> {
        self.option_deref.as_deref_mut()
    }
    #[inline]
    pub fn option_deref_other_style(&self) -> Option<&str> {
        self.option_deref_other_style.as_deref()
    }
    #[inline]
    pub fn option_deref_other_style_mut(&mut self) -> Option<&mut str> {
        self.option_deref_other_style.as_deref_mut()
    }
    #[inline]
    pub fn option_detection(&self) -> Option<&str> {
        self.option_detection.as_deref()
    }
    #[inline]
    pub fn option_detection_mut(&mut self) -> Option<&mut str> {
        self.option_detection.as_deref_mut()
    }
    #[inline]
    pub fn no_option_detection(&self) -> &Option<()> {
        &self.no_option_detection
    }
    #[inline]
    pub fn no_option_detection_mut(&mut self) -> &mut Option<()> {
        &mut self.no_option_detection
    }
    #[inline]
    pub fn option_detection_only_get_mut(&self) -> &Option<()> {
        &self.option_detection_only_get_mut
    }
    #[inline]
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
    #[inline]
    pub fn no_option_detection(&self) -> &Option<()> {
        &self.no_option_detection
    }
    #[inline]
    pub fn no_option_detection_mut(&mut self) -> &mut Option<()> {
        &mut self.no_option_detection
    }
    #[inline]
    pub fn option_deref(&self) -> Option<&str> {
        self.option_deref.as_deref()
    }
    #[inline]
    pub fn option_deref_mut(&mut self) -> &mut Option<String> {
        &mut self.option_deref
    }
    #[inline]
    pub fn field_overrides(&self) -> Option<&str> {
        self.field_overrides.as_deref()
    }
    #[inline]
    pub fn field_overrides_mut(&mut self) -> Option<&mut str> {
        self.field_overrides.as_deref_mut()
    }
    #[inline]
    pub fn option_only_for_get(&self) -> Option<&()> {
        self.option_only_for_get.as_ref()
    }
    #[inline]
    pub fn option_only_for_get_mut(&mut self) -> &mut Option<()> {
        &mut self.option_only_for_get
    }
    #[inline]
    pub fn option_detection_only_get_mut(&self) -> &Option<()> {
        &self.option_detection_only_get_mut
    }
    #[inline]
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
    #[inline]
    pub fn a(&self) -> &Option<String> {
        &self.a
    }
    #[inline]
    pub fn a_mut(&mut self) -> &mut Option<String> {
        &mut self.a
    }
    #[inline]
    pub fn b(&self) -> Option<&String> {
        self.b.as_ref()
    }
    #[inline]
    pub fn b_mut(&mut self) -> Option<&mut String> {
        self.b.as_mut()
    }
    #[inline]
    pub fn c(&self) -> Option<&str> {
        self.c.as_deref()
    }
    #[inline]
    pub fn c_mut(&mut self) -> Option<&mut str> {
        self.c.as_deref_mut()
    }
    #[inline]
    pub fn d(&self) -> &Option<String> {
        &self.d
    }
    #[inline]
    pub fn d_mut(&mut self) -> &mut Option<String> {
        &mut self.d
    }
    #[inline]
    pub fn e(&self) -> Option<&DerefType> {
        self.e.as_deref()
    }
    #[inline]
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
    #[inline]
    pub fn borrow_inner(&self) -> Option<&()> {
        self.borrow_inner.as_ref()
    }
    #[inline]
    pub fn not_borrow_inner(&self) -> &Option<String> {
        &self.not_borrow_inner
    }
}
/// Enum: Option fields with full coverage → Option<&T> (borrow inner applied)
#[fieldwork(get, get_mut)]
enum WithOptions {
    Foo { token: Option<String>, id: u32 },
    Bar { token: Option<String> },
}
impl WithOptions {
    #[inline]
    pub fn id(&self) -> Option<u32> {
        match self {
            Self::Foo { id, .. } => Some(*id),
            _ => None,
        }
    }
    #[inline]
    pub fn id_mut(&mut self) -> Option<&mut u32> {
        match self {
            Self::Foo { id, .. } => Some(id),
            _ => None,
        }
    }
    #[inline]
    pub fn token(&self) -> Option<&str> {
        match self {
            Self::Foo { token, .. } | Self::Bar { token, .. } => token.as_deref(),
        }
    }
    #[inline]
    pub fn token_mut(&mut self) -> Option<&mut str> {
        match self {
            Self::Foo { token, .. } => token.as_deref_mut(),
            Self::Bar { token, .. } => token.as_deref_mut(),
        }
    }
}
/// Enum: option_borrow_inner = false disables unwrapping
#[fieldwork(get, get_mut, option_borrow_inner = false)]
enum NoOptionDetection {
    Foo { data: Option<String> },
    Bar { data: Option<String> },
}
impl NoOptionDetection {
    #[inline]
    pub fn data(&self) -> &Option<String> {
        match self {
            Self::Foo { data, .. } | Self::Bar { data, .. } => data,
        }
    }
    #[inline]
    pub fn data_mut(&mut self) -> &mut Option<String> {
        match self {
            Self::Foo { data, .. } => data,
            Self::Bar { data, .. } => data,
        }
    }
}
/// Enum: opt-in option detection per field
#[fieldwork(get, option_borrow_inner = false)]
enum SelectiveOption {
    Foo {
        #[fieldwork(option_borrow_inner)]
        detected: Option<String>,
        raw: Option<String>,
    },
    Bar { detected: Option<String>, raw: Option<String> },
}
impl SelectiveOption {
    #[inline]
    pub fn detected(&self) -> Option<&str> {
        match self {
            Self::Foo { detected, .. } | Self::Bar { detected, .. } => {
                detected.as_deref()
            }
        }
    }
    #[inline]
    pub fn raw(&self) -> &Option<String> {
        match self {
            Self::Foo { raw, .. } | Self::Bar { raw, .. } => raw,
        }
    }
}
