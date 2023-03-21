use std::sync::{mpsc, Arc, Mutex, RwLock};
use std::thread::JoinHandle;
use std::{thread, time};

use autopilot::mouse;
use gtk::{glib, Application, ApplicationWindow};
use gtk::{prelude::*, Button};

const APP_ID: &str = "dev.al.AutoClicker";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let count = Arc::new(Mutex::new(0));
    let started = Arc::new(RwLock::new(false));

    let button = Button::builder()
        .label("Start")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let (tx, rx) = mpsc::channel();

    button.connect_clicked(move |button| {
        let mut started = started.try_write().unwrap();
        *started = !*started;

        tx.send(*started).unwrap();

        let label = format!("started: {:?}", *started);
        button.set_label(label.as_str());
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Auto Clicker")
        .child(&button)
        .build();

    window.present();
    window.set_focus_visible(true);

    thread::spawn(move || {
        let mut handle: Option<JoinHandle<()>> = None;
        while let Ok(s) = rx.recv() {
            let count = count.clone();
            start_task(count);
        }
    });
}

fn start_task(count: Arc<Mutex<usize>>) -> JoinHandle<()> {
    let intv = time::Duration::from_millis(3000);

    let h = thread::spawn(move || loop {
        mouse::click(mouse::Button::Left, Some(1));
        let mut count = count.lock().unwrap();
        *count += 1;
        println!("Clicked {:?} time(s)", *count);
        thread::sleep(intv);
    });
    h
}
