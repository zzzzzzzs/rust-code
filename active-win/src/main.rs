use std::{thread, time};
// 获取 Windows、MacOS 和 Linux 上活动窗口的位置、大小、标题和一些其他属性
// https://github.com/dimusic/active-win-pos-rs
use active_win_pos_rs::get_active_window;

fn main() {
    thread::sleep(time::Duration::from_secs(1));
    match get_active_window() {
        Ok(active_window) => {
            println!("active window: {:#?}", active_window);
        },
        Err(()) => {
            println!("error occurred while getting the active window");
        }
    }
}