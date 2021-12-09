
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

        let result = Self {
            data: Vec::new(),
            len: capacity,
        };

        return result
    }

    pub fn write(&mut self, _element: T) -> Result<(), Error> {
        unimplemented!("Write the passed element to the CircularBuffer or return FullBuffer error if CircularBuffer is full.");
    }

    pub fn read(&mut self) -> Result<T, Error> {
        // unimplemented!("Read the oldest element from the CircularBuffer or return EmptyBuffer error if CircularBuffer is empty.");

        if self.data.is_empty() {
            return Err(Error::EmptyBuffer)
        }
        else
        {
            match self.data.first().cloned() {
                None => Err(Error::EmptyBuffer),
                Some(t) => Ok(t),
            }
        }
    }

    pub fn clear(&mut self) {
        unimplemented!("Clear the CircularBuffer.");
    }

    pub fn overwrite(&mut self, _element: T) {
        unimplemented!("Write the passed element to the CircularBuffer, overwriting the existing elements if CircularBuffer is full.");
    }
}
