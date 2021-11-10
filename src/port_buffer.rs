use crate::Point;

mod data;

use data::Package;

const LEN: usize = std::mem::size_of::<Package>();

pub struct PortBuffer {
    buffer: [u8; LEN],
    cursor: usize,
}

impl Default for PortBuffer {
    fn default() -> Self {
        Self {
            buffer: [0u8; LEN],
            cursor: 0,
        }
    }
}

impl PortBuffer {
    pub fn as_buf<'a>(&'a mut self) -> &'a mut [u8] {
        &mut self.buffer[self.cursor..]
    }

    pub fn notify_recived<'a>(&'a mut self, n: usize) {
        self.cursor += n;
    }
}

impl Iterator for PortBuffer {
    type Item = Vec<Point>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor == LEN {
            self.cursor = 0;
            if let Some(vec) = Package::decode(&self.buffer) {
                Some(vec)
            } else if let Some(n) = Package::search_head(&self.buffer[1..]) {
                self.buffer.copy_within(n + 1.., 0);
                self.cursor = LEN - n - 1;
                None
            } else {
                None
            }
        } else {
            None
        }
    }
}
