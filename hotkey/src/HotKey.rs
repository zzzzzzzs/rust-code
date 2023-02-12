use device_query::{DeviceQuery, DeviceState, Keycode};
use rdev::{simulate, Button, EventType, Key, SimulateError};
use std::{thread, time};
use active_win_pos_rs::get_active_window;

// 按键映射，目前是识别到 clion ， 保存就编译
fn main() {
    let device_state = DeviceState::new();
    let mut prev_keys = vec![];
    loop {
        let keys = device_state.get_keys();
        if keys != prev_keys {
            // ctrl + s
            if keys.len() == 2 && keys.contains(&Keycode::LControl) && keys.contains(&Keycode::S) {
                println!("ctrl + s 被按下");
                match get_active_window() {
                    Ok(active_window) => {
                        let pro_name = active_window.process_name.to_lowercase();
                        // 按键作用域
                        match pro_name.as_str() {
                            "clion" => {
                                println!("clion 作用域生效");
                                send(&EventType::KeyPress(Key::ControlLeft));
                                send(&EventType::KeyPress(Key::ShiftLeft));
                                send(&EventType::KeyPress(Key::F9));
                                send(&EventType::KeyRelease(Key::F9));
                                send(&EventType::KeyRelease(Key::ShiftLeft));
                                send(&EventType::KeyRelease(Key::ControlLeft));
                            }
                            _ => {
                                println!("目前不支持 {} 作用域", pro_name);
                            }
                        }
                        // println!("active window: {:#?}", active_window);
                    }
                    Err(()) => {
                        println!("error occurred while getting the active window");
                    }
                }
            }
            // println!("{:?}", keys);
        }
        prev_keys = keys;
        // 降低 CPU
        thread::sleep(time::Duration::from_millis(50));
    }
}

fn send(event_type: &EventType) {
    let delay = time::Duration::from_millis(20);
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
    // Let ths OS catchup (at least MacOS)
    thread::sleep(delay);
}