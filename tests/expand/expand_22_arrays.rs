use std::sync::Arc;

#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, get_mut)]
struct DebugStruct {
    array: [u8; 10],
    box_array: Box<[u8; 10]>,
    arc_box_array: Arc<Box<[u8; 10]>>,
    option_array: Option<[u8; 10]>,
    option_box_array: Option<Box<[u8; 10]>>,
    option_arc_box_array: Option<Arc<Box<[u8; 10]>>>,
}

/// Enum: array fields (full and partial coverage)
#[derive(fieldwork::Fieldwork)]
#[fieldwork(get, get_mut)]
enum WithArrays {
    Fixed { data: [u8; 16], extra: [u8; 4] },
    Large { data: [u8; 16] },
}
