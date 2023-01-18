use std::thread;
use std::time::Duration;
use device_query::{DeviceQuery, DeviceState, Keycode};
// CPU 高
// https://github.com/ostrosco/device_query/issues/61
fn main() {
    let device_state = DeviceState::new();
    let mut prev_keys = vec![];
    loop {
        let keys = device_state.get_keys();
        if keys != prev_keys {
            if keys.len() == 2 && keys.contains(&Keycode::LControl) && keys.contains(&Keycode::S) {
                println!("ctrl + s 被按下")
            }
            println!("{:?}", keys);
        }
        prev_keys = keys;
        // 降低 CPU
        thread::sleep(Duration::from_millis(50));
    }
}