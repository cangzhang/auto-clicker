use std::sync::{Arc, Mutex};

use gtk::{glib, Application, ApplicationWindow};
use gtk::{prelude::*, Button};

mod clicker;

const APP_ID: &str = "dev.al.AutoClicker";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let mut mouse_handler = clicker::Clicker::new();
    mouse_handler.init();
    let mouse_handler = Arc::new(Mutex::new(mouse_handler));

    let button = Button::builder()
        .label("Start")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    button.connect_clicked(move |_| {
        let mut c = mouse_handler.lock().unwrap();
        c.toggle();
    });
    button.set_label("Start");

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Auto Clicker")
        .child(&button)
        .build();

    window.present();
    window.set_focus_visible(true);
}
