use device_query::{DeviceEvents, DeviceState};
use std::{thread, time};
// CPU 占用太高了
fn main() {
    let device_state = DeviceState::new();
    let _guard = device_state.on_key_down(|key| {
        println!("Down: {:#?}", key);
    });
    let _guard = device_state.on_key_up(|key| {
        println!("Up: {:#?}", key);
    });

    loop {
        thread::sleep(time::Duration::from_millis(20));
    }
}