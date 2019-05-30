extern crate rumble;
extern crate rand;

use std::thread;
use std::time::Duration;
use rand::{Rng, thread_rng};
use rumble::bluez::manager::Manager;
use rumble::api::{UUID, Central, Peripheral};

pub fn main() {
    let manager = Manager::new().unwrap();

    // get the first bluetooth adapter
    let adapters = manager.adapters().unwrap();
    let mut adapter = adapters.into_iter().nth(0).unwrap();

    // reset the adapter -- clears out any errant state
    adapter = manager.down(&adapter).unwrap();
    adapter = manager.up(&adapter).unwrap();

    // connect to the adapter
    let central = adapter.connect().unwrap();

    // start scanning for devices
    central.start_scan().unwrap();
    // instead of waiting, you can use central.on_event to be notified of
    // new devices
    thread::sleep(Duration::from_secs(2));

    central.peripherals().into_iter().for_each(|p| println!("{:?}", p));

    central.stop_scan().unwrap();
}
