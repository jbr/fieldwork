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
