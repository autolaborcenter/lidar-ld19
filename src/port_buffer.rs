mod data;

use std::collections::VecDeque;

use data::Package;
use lidar::Point;

const LEN: usize = std::mem::size_of::<Package>();

pub struct PortBuffer {
    buffer: [u8; LEN],
    cursor: usize,
    points: VecDeque<Point>,
}

impl Default for PortBuffer {
    fn default() -> Self {
        Self {
            buffer: [0u8; LEN],
            cursor: 0,
            points: VecDeque::new(),
        }
    }
}

impl PortBuffer {
    pub fn as_buf<'a>(&'a mut self) -> &'a mut [u8] {
        &mut self.buffer[self.cursor..]
    }

    pub fn notify_received<'a>(&'a mut self, n: usize) {
        self.cursor += n;
    }
}

impl Iterator for PortBuffer {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(p) = self.points.pop_front() {
            Some(p)
        } else if self.cursor == LEN {
            self.cursor = 0;
            if Package::decode(&self.buffer, |p, r| {
                if r > 120 {
                    self.points.push_back(p);
                }
            }) {
                self.points.pop_front()
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
