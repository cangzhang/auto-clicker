use iced::widget::{button, column, row, text};
use iced::{executor, Alignment, Application, Command, Element, Length, Settings, Theme};

pub fn main() -> iced::Result {
    App::run(Settings::default())
}

#[derive(Debug, Default)]
struct App {
    count: i32,
    running: bool,
    last_time: u64,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncreaseCount,
    Start,
    Stop,
    CheckTask,
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                ..Default::default()
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Auto Clicker")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::IncreaseCount => {
                self.count += 1;
                self.last_time = get_time_in_ms();
            }
            Message::Stop => {
                self.count = 0;
                self.running = false;
            }
            Message::Start => {
                self.running = true;
                self.count = 0;
                self.last_time = get_time_in_ms();
            }
            Message::CheckTask => {
                if !self.running {
                    return Command::none();
                }

                // delay for the first time
                let current_time = get_time_in_ms();
                let intv = current_time - self.last_time;
                if self.count == 0 && intv < 1500 {
                    return Command::none();
                }

                // run every 2000ms
                if intv < 2000 {
                    return Command::none();
                }

                return Command::perform(trigger_click(), |_| Message::IncreaseCount);
            }
        };
        Command::none()
    }

    fn theme(&self) -> Theme {
        Theme::Light
    }

    fn view(&self) -> Element<Message> {
        column![
            row![
                text(self.count).size(50),
                text("  ->  ").size(20),
                text(if self.running { "Y" } else { "N" }).size(30)
            ]
            .align_items(Alignment::Center),
            text(format!("last time: {:?}", self.last_time)).size(30),
            row![
                button("Start").on_press(Message::Start),
                button("Stop").on_press(Message::Stop),
            ]
            .spacing(20)
        ]
        .width(Length::Fill)
        .align_items(Alignment::Center)
        .padding(20)
        .spacing(20)
        .into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::time::every(std::time::Duration::from_millis(100)).map(|_| Message::CheckTask)
    }
}

async fn trigger_click() -> anyhow::Result<()> {
    use autopilot::mouse;

    mouse::click(mouse::Button::Left, Some(1));
    // let handle = tokio::spawn(async move {
    // });
    // handle.await?;
    Ok(())
}

fn get_time_in_ms() -> u64 {
    let current = std::time::SystemTime::now();
    let since_the_epoch = current
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_secs() * 1000 + since_the_epoch.subsec_nanos() as u64 / 1_000_000
}
