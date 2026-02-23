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

## Per-field configuration

`#[field]` and `#[fieldwork]` annotations on enum variant fields configure the
*virtual field* — the generated method covering that field name across all variants.
An annotation on **one** occurrence applies to the entire virtual field; every
variant that structurally has that field participates.

### Opting in from one variant

When there is no item-level `#[fieldwork]` attribute (or when `opt_in` is set),
annotating a single variant's field opts in method generation for the whole virtual
field:

```rust
#[derive(fieldwork::Fieldwork)]
enum Message {
    Request {
        id: u64,
        #[field(get, set)]
        priority: u8,
    },
    Response { id: u64, priority: u8 },
    Heartbeat { id: u64, priority: u8 },
}
```

```rust
// GENERATED
# enum Message { Request { id: u64, priority: u8 }, Response { id: u64, priority: u8 }, Heartbeat { id: u64, priority: u8 }, }
impl Message {
    pub fn priority(&self) -> u8 {
        match self {
            Self::Request { priority, .. }
            | Self::Response { priority, .. }
            | Self::Heartbeat { priority, .. } => *priority,
        }
    }
    pub fn set_priority(&mut self, priority: u8) -> &mut Self {
        match self {
            Self::Request { priority: priority_binding, .. } => {
                *priority_binding = priority;
            }
            Self::Response { priority: priority_binding, .. } => {
                *priority_binding = priority;
            }
            Self::Heartbeat { priority: priority_binding, .. } => {
                *priority_binding = priority;
            }
        }
        self
    }
}

```

`id` has no annotation and generates no methods. `priority` is in all three variants
(full coverage), so both `priority() -> u8` and `set_priority()` are generated.

### Vetoing a field

`#[field = false]` on **any** occurrence globally suppresses all methods for that
virtual field, even in variants where the field is otherwise accessible:

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set)]
enum State {
    Normal { value: i32 },
    Transitioning { value: i32 },
    Debug {
        #[field = false]
        value: i32,
    },
}
```

```rust
// GENERATED
# enum State { Normal { value: i32 }, Transitioning { value: i32 }, Debug { value: i32 }, }
impl State {}

```

`Debug`'s veto on `value` suppresses all `value()` and `set_value()` methods, even
though `Normal` and `Transitioning` have a perfectly accessible `value` field.

### Vetoing a specific method

`#[field(get = false)]` (or `set = false`, etc.) on any occurrence suppresses that
method while leaving others intact:

```rust
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, set)]
enum Sensor {
    Active { reading: f32 },
    Calibrating { reading: f32 },
    Fault {
        #[field(get = false)]
        reading: f32,
    },
}
```

```rust
// GENERATED
# enum Sensor { Active { reading: f32 }, Calibrating { reading: f32 }, Fault { reading: f32 }, }
impl Sensor {
    pub fn set_reading(&mut self, reading: f32) -> &mut Self {
        match self {
            Self::Active { reading: reading_binding, .. } => {
                *reading_binding = reading;
            }
            Self::Calibrating { reading: reading_binding, .. } => {
                *reading_binding = reading;
            }
            Self::Fault { reading: reading_binding, .. } => {
                *reading_binding = reading;
            }
        }
        self
    }
}

```

`set_reading()` is generated with full coverage across all three variants;
`reading()` is suppressed because the `Fault` variant's reading should not be
read directly.

### Rules

**One annotation per virtual field.** At most one *substantive* `#[field]`
annotation (anything beyond a rename) may appear per virtual field. Fieldwork
reports a compile error if more than one occurrence is annotated.

**Rename annotations are an exception.** `#[field = "name"]` and
`#[fieldwork(rename = name)]` may appear on all occurrences, since they control
which variants are grouped into the same virtual field — every occurrence that
should participate in the renamed virtual field needs to carry the matching rename.

**Type consistency.** If a field has different types across variants, fieldwork
silently omits all methods for it. Annotating such a field with `#[field]` is a
compile error that identifies the conflicting types.

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
