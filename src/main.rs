use std::sync::{Arc, Mutex};
use std::{thread, time};

use autopilot::mouse;
use gtk::glib::{MainContext, PRIORITY_DEFAULT, clone};
use gtk::{glib, Application, ApplicationWindow};
use gtk::{prelude::*, Button};

const APP_ID: &str = "dev.al.AutoClicker";

enum Message {
    ToggleTaskStatus,
}

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let count = Arc::new(Mutex::new(0));
    let running = Arc::new(Mutex::new(false));

    let (sender, receiver) = MainContext::channel(PRIORITY_DEFAULT);

    let button = Button::builder()
        .label("Start")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let running_ui = running.clone();
    // let count_ui = count.clone();
    button.connect_clicked(move |_| {
        let mut running = running_ui.lock().unwrap();
        *running = !*running;

        sender
            .send(Message::ToggleTaskStatus)
            .expect("Send ToggleCommand failed");
    });
    button.set_label("Start");

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Auto Clicker")
        .child(&button)
        .build();

    window.present();
    window.set_focus_visible(true);

    thread::spawn(move || loop {
        {
            let running = running.lock().unwrap();
            let mut count = count.lock().unwrap();
            if *running {
                mouse::click(mouse::Button::Left, Some(1));
                *count += 1;
            } else {
                *count = 0;
            }
            println!("running: {:?}, count: {:?}", *running, *count);
        }

        thread::sleep(time::Duration::from_secs(1));
    });

    receiver.attach(
        None,
        clone!(@weak button => @default-return Continue(false),
                    move |msg| {
                        match msg {
                            Message::ToggleTaskStatus => {
                                Continue(true)
                            }
                        }
                    }
        ),
    );
}
