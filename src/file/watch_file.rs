use std::{thread::sleep, time::Duration};

fn main() {
    use hotwatch::{Event, Hotwatch};

    let mut hotwatch = Hotwatch::new().expect("hotwatch failed to initialize!");
    hotwatch
        .watch("C:/test.txt", |event: Event| {
            println!("get some event {:?}",event);
            if let Event::Write(path) = event {
                if path.file_name().unwrap().to_str().unwrap().eq("test.txt") {
                    println!("test.txt has changed.{:?}",path);
                }
                else{
                    println!("other file has been changed!{:?}",path);
                }
            }
        })
        .expect("failed to watch file!");

    loop {
        sleep(Duration::from_secs(2));
    }
}