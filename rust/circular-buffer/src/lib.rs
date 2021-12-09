
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
        /* a CircularBuffer constructor */
        let mut ring_buffer = CircularBuffer {
            data: Vec::new(),
            len: capacity,
        };

        /* reserve space for elements */
        ring_buffer.data.reserve(capacity);

        return ring_buffer
    }

    pub fn write(&mut self, _element: T) -> Result<(), Error> {
        /* Write the passed element to the CircularBuffer
         * Otherwise raise FullBuffer error */
        if self.data.len() == self.len {
            return Err(Error::FullBuffer)
        }
        else {
            self.data.push(_element);
            return Ok(())
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        /*
         * Raise error if buffer is empty
         * Otherwise return oldest value
         */
        match self.data.first().cloned() {
            None => Err(Error::EmptyBuffer),
            Some(t) => Ok(self.data.remove(0)),
        }
    }

    pub fn clear(&mut self) {
        // unimplemented!("Clear the CircularBuffer.");
        self.data.clear()
    }

    pub fn overwrite(&mut self, _element: T) {
        unimplemented!("Write the passed element to the CircularBuffer, overwriting the existing elements if CircularBuffer is full.");
    }
}
