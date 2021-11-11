use serial_port::{Port, PortKey, SerialPort};
use std::time::Duration;

mod port_buffer;

use port_buffer::PortBuffer;

pub use lidar::{driver, Config, Lidar, LidarDriver, Point};

const POINT_RECEIVE_TIMEOUT: Duration = Duration::from_millis(200);
const OPEN_TIMEOUT: Duration = Duration::from_secs(1);
const POINT_PARSE_TIMEOUT: Duration = Duration::from_millis(250);

pub const CONFIG: Config = Config {
    len_meter: 1000,
    dir_round: 36000,
    zipped_size: 4,
};

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
        CONFIG.dir_round
    }

    fn new(key: &Self::Key) -> Option<Self> {
        Port::open(key, 230400, POINT_RECEIVE_TIMEOUT.as_millis() as u32)
            .ok()
            .map(|port| Self {
                port,
                buffer: Default::default(),
            })
    }

    fn receive(&mut self) -> bool {
        self.port
            .read(self.buffer.as_buf())
            .filter(|n| *n > 0)
            .map(|n| self.buffer.notify_received(n))
            .is_some()
    }

    fn parse(&mut self) -> Option<lidar::Point> {
        self.buffer.next()
    }
}

#[inline]
pub const fn zip(p: Point) -> [u8; CONFIG.zipped_size] {
    [
        (p.len >> 8) as u8,
        p.len as u8,
        (p.dir >> 8) as u8,
        p.dir as u8,
    ]
}

#[inline]
pub const unsafe fn unzip(buf: &[u8]) -> Point {
    Point {
        len: ((buf[0] as u16) << 8) | buf[1] as u16,
        dir: ((buf[2] as u16) << 8) | buf[3] as u16,
    }
}

#[test]
fn assert_assign() {
    // 随便的一组值
    const P0: Point = Point {
        len: 999,
        dir: 7777,
    };
    unsafe { assert_eq!(unzip(&zip(P0)), P0) };

    // 设备可能的最大值
    const P1: Point = Point {
        len: 12000,
        dir: 35999,
    };
    unsafe { assert_eq!(unzip(&zip(P1)), P1) };

    // 数据结构支持的最大值
    const P2: Point = Point {
        len: 65535,
        dir: 65535,
    };
    unsafe { assert_eq!(unzip(&zip(P2)), P2) };
}
