#[derive(Debug)]
pub struct Rom {
    pub contents: Vec<u8>,
}

impl Rom {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            contents: Vec::with_capacity(capacity)
        }
    }

    pub fn consume_bytes(&mut self, bytes: &mut Vec<u8>) {
        let len = self.contents.capacity();
        let take_len = len.min(bytes.len());

        let taken: Vec<u8> = bytes.drain(0..take_len).collect();
        self.contents.extend(taken);
    }

    pub fn read(&self, addr: u16) -> Option<u8> {
        self.contents.get(addr as usize).map(|b| b.clone())
    }
}
