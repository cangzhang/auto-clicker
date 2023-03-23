use std::sync::{Arc, Mutex};

use gtk::glib::{MainContext, PRIORITY_DEFAULT};
use gtk::{glib, Application, ApplicationWindow};
use gtk::{prelude::*, Button};

mod clicker;

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
    let mut mouse_handler = clicker::Clicker::new();
    mouse_handler.init();
    let mouse_handler = Arc::new(Mutex::new(mouse_handler));

    let (sender, receiver) = MainContext::channel(PRIORITY_DEFAULT);

    let button = Button::builder()
        .label("Start")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    button.connect_clicked(move |_| {
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

    receiver.attach(None, move |msg| {
        match msg {
            Message::ToggleTaskStatus => {
                let mut handler = mouse_handler.lock().unwrap();
                handler.toggle();
            }
        }
        Continue(true)
    });
}
