use lidar::LidarDriver;
use serial_port::{Port, PortKey, SerialPort};
use std::time::Duration;

mod port_buffer;
use port_buffer::PortBuffer;

const POINT_RECEIVE_TIMEOUT: Duration = Duration::from_millis(200);
const POINT_PARSE_TIMEOUT: Duration = Duration::from_millis(250);
const OPEN_TIMEOUT: Duration = Duration::from_secs(1);

pub const MAX_DIR: u16 = 36000;

pub struct LD19 {
    port: Port,
    buffer: PortBuffer,
}

impl LidarDriver for LD19 {
    type Key = PortKey;

    fn keys() -> Vec<Self::Key> {
        Port::list().into_iter().map(|id| id.key).collect()
    }

    fn open_timeout() -> Duration {
        OPEN_TIMEOUT
    }

    fn parse_timeout() -> Duration {
        POINT_PARSE_TIMEOUT
    }

    fn max_dir() -> u16 {
        MAX_DIR
    }

    fn new(key: &Self::Key) -> Option<Self> {
        if let Ok(port) = Port::open(key, 230400, POINT_RECEIVE_TIMEOUT.as_millis() as u32) {
            Some(LD19 {
                port,
                buffer: Default::default(),
            })
        } else {
            None
        }
    }

    fn receive(&mut self) -> bool {
        match self.port.read(self.buffer.as_buf()) {
            Some(n) => {
                if n == 0 {
                    return false;
                } else {
                    self.buffer.notify_recived(n);
                }
            }
            None => return false,
        }
        true
    }

    fn parse(&mut self) -> Option<lidar::Point> {
        self.buffer.next()
    }
}
