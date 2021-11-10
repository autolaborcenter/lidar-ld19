use crate::Point;

pub(super) struct SectionCollector {
    dir_each: u16,
    current: u8,
    buffer: Vec<Point>,
}

impl SectionCollector {
    pub fn new(len: u8) -> Self {
        let dir_each = crate::MAX_DIR / len as u16;
        Self {
            dir_each,
            current: 0,
            buffer: vec![],
        }
    }

    pub fn push(&mut self, mut p: Vec<Point>) -> Option<(u8, Vec<Point>)> {
        let i = (p[0].dir / self.dir_each) as u8;
        let result = if self.current == i {
            None
        } else {
            Some((
                std::mem::replace(&mut self.current, i),
                std::mem::replace(
                    &mut self.buffer,
                    Vec::with_capacity(self.dir_each as usize / 10),
                ),
            ))
        };

        if p.len() > 0 {
            self.buffer.append(&mut p);
        }
        result
    }
}
