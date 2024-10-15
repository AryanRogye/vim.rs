use crate::list::linked_list::LinkedList;



#[derive(Clone)]
pub struct RingBuffer{
    buffer      : LinkedList<String>,       // Buffer For Command History
    capacity    : usize,             // Max Capacity
    head        : usize,             // Head Index
    size        : usize,             // curr size
}

impl RingBuffer {
    pub fn new(capacity : usize) -> Self {
        RingBuffer{
            buffer      : LinkedList::new(),
            capacity    : capacity.clone(),
            head        : 0,
            size        : 0,
        }
    }
    // Getters
    pub fn get_buffer(&self) -> LinkedList<String> { self.buffer.clone() }
    pub fn get_capacity(&self) -> usize { self.capacity.clone() }
    pub fn get_head(&self) -> usize { self.capacity.clone() }
    pub fn get_size(&self) -> usize { self.capacity.clone() }
    // Setters
    pub fn add_to_buffer(&self, value : String) {
        if self.get_size() > self.get_capacity() {
        
        }
    }
}
