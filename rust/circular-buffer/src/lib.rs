
pub struct CircularBuffer<T> {
    data: Vec<T>,
    len: usize,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T: Clone> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        // a CircularBuffer constructor
        return Self {
            data: Vec::new(),
            len: capacity,
        }
    }

    pub fn write(&mut self, _element: T) -> Result<(), Error> {
        unimplemented!("Write the passed element to the CircularBuffer or return FullBuffer error if CircularBuffer is full.");
    }

    pub fn read(&mut self) -> Result<T, Error> {
        /*
         * Raise error if buffer is empty
         * Otherwise return oldest value
         */
        match self.data.first().cloned() {
            None => Err(Error::EmptyBuffer),
            Some(t) => Ok(t),
        }
    }

    pub fn clear(&mut self) {
        unimplemented!("Clear the CircularBuffer.");
    }

    pub fn overwrite(&mut self, _element: T) {
        unimplemented!("Write the passed element to the CircularBuffer, overwriting the existing elements if CircularBuffer is full.");
    }
}
