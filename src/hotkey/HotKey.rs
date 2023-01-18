use windows_hotkeys::{
    keys::{ModKey, VKey},
    HotkeyManager,
};
use active_win_pos_rs::get_active_window;
use std::{thread, time};
use rdev::{EventType, simulate, Key};

// TODO 目前原始按键功能就不能用了，还是继续使用 AutoHotKey 吧

/// A simple control flow enum that can either continue the event handler or stop it
enum ControlFlow {
    /// Continue the handler
    Continue,
    /// Exit (stop) the handler
    Exit,
}

fn main() {
    // The HotkeyManager is generic over the return type of the callback functions. So if the
    // callbacks return data, it is available in the event loop and can be used to determin further
    // actions
    let mut hkm = HotkeyManager::new();


    // A hotkey for CTRL + ALT + A, that will just keep on running and not break the loop
    hkm.register(VKey::S, &[ModKey::Ctrl], || {
        println!("Pressed CTRL + S");

        // let mut enigo = Enigo::new();
        // 保证 ctrl + s 被按下
        // enigo.key_sequence("s");
        // enigo.key_down(Key::Control);
        // enigo.key_click(Key::Layout('s'));
        // enigo.key_up(Key::Control);
        // enigo.key_down(Key::Control);
        // enigo.key_click(Key::Layout('s'));
        // enigo.key_up(Key::Control);
        // 模拟按下 ctrl + shift+ F9
        // enigo.key_down(Key::Control);
        // enigo.key_down(Key::Meta);
        // enigo.key_click(Key::F9);
        // enigo.key_up(Key::Control);
        // enigo.key_up(Key::Meta);
        match get_active_window() {
            Ok(active_window) => {
                let pro_name = active_window.process_name.to_lowercase();
                // 按键作用域
                match pro_name.as_str() {
                    "clion" => {
                        println!("clion 作用域生效");
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
        // Set the control flow to keep running
        ControlFlow::Continue
    })
        .unwrap();

    // A hotkey for CTRL + ALT + C, that will break and stop the loop
    // hkm.register(VKey::C, &[ModKey::Ctrl, ModKey::Alt], || {
    //     println!("Pressed CTRL + ALT + C");
    //     println!("Breaking the loop");
    //
    //     // Set the control flow to exit
    //     ControlFlow::Exit
    // })
    //     .unwrap();

    loop {
        // Handle one hotkey event. This will block until a hotkey event is triggered and return
        // the return value of the callback
        let control_flow = hkm.handle_hotkey();

        // Since the callbacks return a `ControlFlow` variant, check if the loop should exit
        match control_flow {
            Some(ControlFlow::Exit) | None => break,
            _ => (),
        }
    }

    println!("Loop exited");
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