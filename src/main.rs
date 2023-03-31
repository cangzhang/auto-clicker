use std::sync::{Arc, Mutex};
use std::{thread, time};

use autopilot::mouse;
use gtk::glib::{clone, MainContext, PRIORITY_DEFAULT};
use gtk::{glib, Application, ApplicationWindow, Box, Entry, EntryBuffer, InputPurpose, Label};
use gtk::{prelude::*, Button};

const APP_ID: &str = "dev.al.AutoClicker";

enum Message {
    UpdateCountText(bool, usize),
}

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let count = Arc::new(Mutex::new(0));
    let running = Arc::new(Mutex::new(false));
    let intv_default: u64 = 3;
    let intv = Arc::new(Mutex::new(intv_default));

    let (sender, receiver) = MainContext::channel(PRIORITY_DEFAULT);

    let button = Button::builder()
        .label("Start")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let label = Label::new(None);
    label.set_text("");

    let running_ui = running.clone();
    button.connect_clicked(move |btn| {
        let mut running = running_ui.lock().unwrap();
        *running = !*running;
        btn.set_label(if *running { "Stop" } else { "Start" });
    });
    button.set_label("Start");

    let text_buffer = EntryBuffer::builder().text("3").build();
    let input = Entry::with_buffer(&text_buffer);
    text_buffer.set_text("3");
    input.set_margin_start(10);
    input.set_margin_end(10);
    input.set_input_purpose(InputPurpose::Number);
    
    let intv_ui = intv.clone();
    input.connect_changed(move |input| {
        let new_text = input.text();
        let filtered_text: String = new_text.chars().filter(|c| c.is_numeric()).collect();
        if new_text != filtered_text {
            text_buffer.set_text(&filtered_text);
        }

        println!("filtered: {filtered_text}");
        if let Ok(seconds) = filtered_text.parse::<u64>() {
            let mut intv = intv_ui.lock().unwrap();
            *intv = seconds;
        }
    });

    let vbox = Box::new(gtk::Orientation::Vertical, 10);
    vbox.append(&input);
    vbox.append(&label);
    vbox.append(&button);

    vbox.set_margin_top(20);
    vbox.set_margin_bottom(10);

    let window = ApplicationWindow::builder()
        .application(app)
        .default_height(100)
        .default_width(200)
        .title("Auto Clicker")
        .child(&vbox)
        .build();

    window.present();
    window.set_focus_visible(true);

    let intv_thread = intv.clone();
    thread::spawn(move || loop {
        {
            let running = running.lock().unwrap();
            let mut count = count.lock().unwrap();
            if *running {
                mouse::click(mouse::Button::Left, Some(1));
                *count += 1;
            }
            sender
                .send(Message::UpdateCountText(*running, *count))
                .unwrap();
        }

        let intv = intv_thread.lock().unwrap();
        let intv = if *intv > 1 { *intv } else { 1 };
        thread::sleep(time::Duration::from_secs(intv));
    });

    receiver.attach(
        None,
        clone!(@weak button => @default-return Continue(false),
                    move |msg| {
                        match msg {
                            Message::UpdateCountText(status, count) => {
                                let text = if status {
                                    format!("Status: Running, Count: {count} times")
                                } else {
                                    format!("Status: Stopped")
                                };

                                label.set_text(&text);
                            },
                        }
                        Continue(true)
                    }
        ),
    );
}
