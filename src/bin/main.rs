use lidar::{
    driver::{Indexer, SupervisorEventForMultiple::*, SupervisorForMultiple},
    Lidar,
};
use lidar_ld19::LD19;
use std::time::{Duration, Instant};

fn main() {
    let mut indexer = Indexer::new(2);
    SupervisorForMultiple::<Lidar<LD19>>::new().join(2, |e| {
        match e {
            Connected(k, _) => {
                println!("connected: COM{}", &k);
                indexer.add(k.clone());
            }
            ConnectFailed {
                current,
                target,
                next_try,
            } => {
                println!("{}/{}", current, target);
                *next_try = Instant::now() + Duration::from_secs(1);
            }
            Event(k, Some((_, (_, _))), _) => if let Some(_) = indexer.find(&k) {},
            Event(_, _, _) => {}
            Disconnected(k) => {
                println!("disconnected: COM{}", &k);
                indexer.remove(&k);
            }
        }
        2
    });
}
