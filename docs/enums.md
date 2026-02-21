# Enum support

`#[derive(Fieldwork)]` works on enums with the same syntax as structs. All
field-accessor method types (`get`, `get_mut`, `set`, `with`, `without`,
`take`, `into_field`) are supported and generate `match`-based implementations
across all variants.

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set, into_field)]
enum Status {
    Active { name: String },
    Inactive { name: String },
}
```

```rust
// GENERATED
# enum Status { Active { name: String }, Inactive { name: String }, }
impl Status {
    pub fn name(&self) -> &str {
        match self {
            Self::Active { name, .. } | Self::Inactive { name, .. } => &**name,
        }
    }
    pub fn set_name(&mut self, name: String) -> &mut Self {
        match self {
            Self::Active { name: name_binding, .. } => {
                *name_binding = name;
            }
            Self::Inactive { name: name_binding, .. } => {
                *name_binding = name;
            }
        }
        self
    }
    pub fn into_name(self) -> String {
        match self {
            Self::Active { name, .. } | Self::Inactive { name, .. } => name,
        }
    }
}

```

---

## Full and partial coverage

The central concept for enum field methods is **coverage**: whether a field
name appears in **all** variants or only **some**.

| Coverage | `get` / `get_mut` | `set`, `with`, `without`, `take`, `into_field` |
|---|---|---|
| Full (field in every variant) | Same return type as struct fields | Generated normally |
| Partial (field in some variants) | Return type wrapped in `Option` | Not generated |

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set)]
enum Event {
    Click { x: i32, y: i32, button: u8 },
    Move  { x: i32, y: i32 },
    Scroll { x: i32, y: i32, delta: f32 },
}
```

```rust
// GENERATED
# enum Event { Click { x: i32, y: i32, button: u8 }, Move { x: i32, y: i32 }, Scroll { x: i32, y: i32, delta: f32 }, }
impl Event {
    pub fn button(&self) -> Option<u8> {
        match self {
            Self::Click { button, .. } => Some(*button),
            _ => None,
        }
    }
    pub fn delta(&self) -> Option<f32> {
        match self {
            Self::Scroll { delta, .. } => Some(*delta),
            _ => None,
        }
    }
    pub fn x(&self) -> i32 {
        match self {
            Self::Click { x, .. } | Self::Move { x, .. } | Self::Scroll { x, .. } => *x,
        }
    }
    pub fn set_x(&mut self, x: i32) -> &mut Self {
        match self {
            Self::Click { x: x_binding, .. } => {
                *x_binding = x;
            }
            Self::Move { x: x_binding, .. } => {
                *x_binding = x;
            }
            Self::Scroll { x: x_binding, .. } => {
                *x_binding = x;
            }
        }
        self
    }
    pub fn y(&self) -> i32 {
        match self {
            Self::Click { y, .. } | Self::Move { y, .. } | Self::Scroll { y, .. } => *y,
        }
    }
    pub fn set_y(&mut self, y: i32) -> &mut Self {
        match self {
            Self::Click { y: y_binding, .. } => {
                *y_binding = y;
            }
            Self::Move { y: y_binding, .. } => {
                *y_binding = y;
            }
            Self::Scroll { y: y_binding, .. } => {
                *y_binding = y;
            }
        }
        self
    }
}

```

`x` and `y` appear in all three variants — full coverage. `button` and `delta`
appear in only one variant each — partial coverage.

- Full-coverage `get`: exhaustive or-pattern match, same smart defaults as struct fields (Copy by value, `String` → `&str`, etc.)
- Partial-coverage `get`: single match arm plus `_ => None`, return value wrapped in `Option`
- Full-coverage `set`: exhaustive match, returns `&mut Self` by default
- Partial-coverage `set`: **not generated currently**

---

## `into_field`

Full-coverage fields generate a method that consumes `self` and returns the
field value via an exhaustive or-pattern. Partial-coverage fields are not
generated.

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(into_field)]
enum Status {
    Active { name: String },
    Inactive { name: String },
}
```

```rust
// GENERATED
# enum Status { Active { name: String }, Inactive { name: String }, }
impl Status {
    pub fn into_name(self) -> String {
        match self {
            Self::Active { name, .. } | Self::Inactive { name, .. } => name,
        }
    }
}

```

---

## Tuple variants

Unnamed tuple fields must be given a name via `#[field = name]` to participate
in any method, consistent with how tuple structs work:

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get)]
enum Color {
    Rgb(#[field = "r"] u8, #[field = "g"] u8, #[field = "b"] u8),
    Named(#[field = "name"] String),
    Transparent,
}
```

```rust
// GENERATED
# enum Color { Rgb (u8, u8, u8), Named (String), Transparent, }
impl Color {
    pub fn b(&self) -> Option<u8> {
        match self {
            Self::Rgb(_, _, b, ..) => Some(*b),
            _ => None,
        }
    }
    pub fn g(&self) -> Option<u8> {
        match self {
            Self::Rgb(_, g, ..) => Some(*g),
            _ => None,
        }
    }
    pub fn name(&self) -> Option<&str> {
        match self {
            Self::Named(name, ..) => Some(&**name),
            _ => None,
        }
    }
    pub fn r(&self) -> Option<u8> {
        match self {
            Self::Rgb(r, ..) => Some(*r),
            _ => None,
        }
    }
}

```

`r`, `g`, `b`, and `name` are all partial-coverage (not in every variant), so
`get` returns `Option` for each.

---

## `#[variant(...)]`

The `#[variant(...)]` attribute annotates enum variants, analogous to
`#[field(...)]` on struct fields.

`#[variant(skip)]` excludes a variant from all method generation. Skipped
variants are still counted in the total when calculating coverage, so they can
turn otherwise full-coverage fields into partial-coverage fields:

```rust

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set)]
enum Message {
    Text { content: String },
    Image { url: String },
    #[variant(skip)]
    Hidden { data: String },
}

```

```rust
// GENERATED
# enum Message { Text { content: String }, Image { url: String }, Hidden { data: String }, }
impl Message {
    pub fn content(&self) -> Option<&str> {
        match self {
            Self::Text { content, .. } => Some(&**content),
            _ => None,
        }
    }
    pub fn url(&self) -> Option<&str> {
        match self {
            Self::Image { url, .. } => Some(&**url),
            _ => None,
        }
    }
}

```

`Hidden` contributes to the total variant count (3) but is excluded from method
generation. `content` and `url` each appear in only one of the three variants each, making them
partial-coverage: both return `Option<&str>` and no setters are generated.

---

## Configuration

Enum configuration follows the same cascade as structs, with `#[variant(...)]`
slotting between the item-method and field levels. The most specific level wins.

| Level | Syntax | Scope |
|---|---|---|
| Item | `#[fieldwork(copy = false)]` | All methods, all variants |
| Item-method | `#[fieldwork(get(copy = false))]` | One method type, all variants |
| Variant | `#[variant(skip)]` | All methods for this variant |
| Field | `#[field(deref = false)]` | All methods for this field occurrence |
| Field-method | `#[field(get(copy = false))]` | One method on this field occurrence |

See [`configuration`](crate::configuration) for general cascade rules.
