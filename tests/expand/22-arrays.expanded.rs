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
        self.option_box_array.as_mut().map(|option_box_array| &mut option_box_array[..])
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
