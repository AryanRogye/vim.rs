use std::error::Error;

use super::ring_buffer::RingBuffer;

pub struct Buffer {
    command_history : RingBuffer,
    file_lines      : Vec<String>,
}

// file_lines is gonna be a implimentation of a LinkedList<String>
// the LinkedList is coming from
// the cpp file in list/Linked_list.cpp and Linked_list.hpp

impl Buffer {
    pub fn new() -> Self {
        Buffer{
            command_history : RingBuffer::new(5),
            file_lines      : Vec::new(),
        }
    }

    pub fn configure_lines(&self, content : String) -> Result<(), Box<dyn Error>>{
        // Basically Each line needs to get stored in file_lines
        Ok(())
    }
}
