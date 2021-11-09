use std::time::Duration;

use driver::{SupersivorEventForSingle, SupervisorForSingle};
use lidar_ld19::LD19;

fn main() {
    SupervisorForSingle::<LD19>::new().join(|event| match event {
        SupersivorEventForSingle::Connected(_, _) => {
            print!("Connected!");
            true
        }
        SupersivorEventForSingle::Disconnected => {
            println!("DisConnected!");
            true
        }
        SupersivorEventForSingle::Event(_, e) => {
            if let Some((_,(i,_))) = e{

                println!("{}", i );
            }
            
            true
        }
        SupersivorEventForSingle::ConnectFailed => {
            std::thread::sleep(Duration::from_secs(1));
            println!("Connected Failed!");
            true
        }
    })
}