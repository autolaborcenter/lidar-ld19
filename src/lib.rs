use driver::Driver;
use point::Point;
use port_buffer::PortBuffer;
use section_collector::SectionCollector;
use serial_port::{Port, PortKey, SerialPort};
use std::time::{Duration, Instant};

const POINT_RECEIVE_TIMEOUT: Duration = Duration::from_millis(200);
const POINT_PARSE_TIMEOUT: Duration = Duration::from_millis(250);
const OPEN_TIMEOUT: Duration = Duration::from_secs(3);

mod point;
mod port_buffer;
mod section_collector;

pub struct LD19 {
    port: Port,
    buffer: PortBuffer<47>,
    last_time: Instant,
    section: SectionCollector,
    filter: fn(Point) -> bool,
}

impl Driver for LD19 {
    type Pacemaker = ();
    type Key = PortKey;
    type Event = (u8, Vec<Point>);

    fn keys() -> Vec<Self::Key> {
        Port::list().into_iter().map(|id| id.key).collect()
    }

    fn open_timeout() -> Duration {
        OPEN_TIMEOUT
    }

    fn new(key: &Self::Key) -> Option<(Self::Pacemaker, Self)> {
        match Port::open(key, 230400, POINT_RECEIVE_TIMEOUT.as_millis() as u32) {
            Ok(port) => {
                println!("PortOpen!");
                Some((
                    (),
                    LD19 {
                        port,
                        buffer: Default::default(),
                        last_time: Instant::now(),
                        section: SectionCollector::new(8),
                        filter: |_| true,
                    },
                ))
            }
            Err(_) => None,
        }
    }

    fn join<F>(&mut self, mut f: F) -> bool
    where
        F: FnMut(&mut Self, Option<(Instant, Self::Event)>) -> bool,
    {
        let mut time = Instant::now();
        loop {
            if let Some(v) = self.buffer.next() {
                time = self.last_time;
                let v = v.into_iter().filter(|p| (self.filter)(*p)).collect();
                if let Some(section) = self.section.push(v) {
                    if !f(self, Some((time, section))) {
                        return true;
                    }
                }
            } else if self.last_time > time + POINT_PARSE_TIMEOUT {
                return false;
            } else {
                match self.port.read(self.buffer.as_buf()) {
                    Some(n) => {
                        if n == 0 {
                            return false;
                        } else {
                            self.last_time = Instant::now();
                            self.buffer.notify_recived(n);
                        }
                    }
                    None => return false,
                }
            }
        }
    }
}
