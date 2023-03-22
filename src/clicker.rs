use std::{
    sync::{Arc, Mutex},
    thread, time,
};

use autopilot::mouse;

pub struct Clicker {
    pub running: Arc<Mutex<bool>>,
    pub count: Arc<Mutex<usize>>,
}

impl Clicker {
    pub fn new() -> Self {
        Self {
            running: Arc::new(Mutex::new(false)),
            count: Arc::new(Mutex::new(0)),
        }
    }

    pub fn init(&mut self) {
        let count = self.count.clone();
        let running = self.running.clone();

        thread::spawn(move || loop {
            let running = running.lock().unwrap();
            let mut count = count.lock().unwrap();

            if *running {
                mouse::click(mouse::Button::Left, Some(1));
                *count += 1;
                println!("clicked {:?} times", *count);
                thread::sleep(time::Duration::from_millis(3000));
            } else {
                *count = 0;
                thread::sleep(time::Duration::from_millis(1000));
            }
        });
    }

    pub fn toggle(&mut self) {
        let running = self.running.clone();
        let mut running = running.lock().unwrap();
        *running = !*running;
        println!("Next: {:?}", *running);
    }
}
