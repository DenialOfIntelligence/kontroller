use iced::widget::container;
use iced::widget::Column;
use iced::widget::{button, text_input, Slider};
use iced::{executor, Alignment, Application, Command, Element, Length, Renderer, Settings, Theme};
use kontroller::post;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;

pub fn main() -> iced::Result {
    State::run(Settings {
        exit_on_close_request: true,
        ..Settings::default()
    })
}

struct State {
    addr: String,
    speed: f64,
    tx: Sender<Task>,
}

impl Default for State {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let mut addr = String::new();
            let mut speed: f64;
            for tasks in rx {
                match tasks {
                    Task::FW => {
                        if !addr.is_empty() {
                            post(&addr, "state", "1");
                            println!("Recieved FW ");
                        }
                    }

                    Task::BW => {
                        if !addr.is_empty() {
                            post(&addr, "state", "2");
                            println!("Recieved BW");
                        }
                    }

                    Task::OFF => {
                        if !addr.is_empty() {
                            post(&addr, "state", "3");
                            println!("Recieved OFF")
                        }
                    }

                    Task::Addr(s) => {
                        addr = s;
                        println!("Changed addr to {}", &addr);
                    }

                    Task::Speed(i) => {
                        speed = i;
                        if !addr.is_empty() {
                            post(&addr, "speed", speed);
                            println!("Changed speed to {}", &speed.round());
                        }
                    }
                }
            }
        });

        State {
            addr: String::default(),
            speed: f64::default(),
            tx,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Addr(String),
    FW,
    BW,
    Stop,
    Speed(f64),
}

enum Task {
    FW,
    BW,
    OFF,
    Speed(f64),
    Addr(String),
}

impl Application for State {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (State, Command<Message>) {
        (State::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Click counter")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::FW => {
                self.tx.send(Task::FW).unwrap();
                Command::none()
            }
            Message::BW => {
                self.tx.send(Task::BW).unwrap();
                Command::none()
            }
            Message::Stop => {
                self.tx.send(Task::OFF).unwrap();
                Command::none()
            }
            Message::Addr(addr) => {
                self.tx.send(Task::Addr(addr.clone())).unwrap();
                self.addr = addr;
                Command::none()
            }
            Message::Speed(speed) => {
                self.tx.send(Task::Speed(self.speed)).unwrap();
                self.speed = speed;
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let input = text_input("ip with /motor attached", &self.addr)
            .on_input(Message::Addr)
            .padding(30)
            .width(200);

        let fw_button: iced::widget::Button<'_, Message, Renderer> =
            button("Forwords").on_press(Message::FW).padding(40);

        let bw_button: iced::widget::Button<'_, Message, Renderer> =
            button("Backwords").on_press(Message::BW).padding(40);

        let stop_button: iced::widget::Button<'_, Message, Renderer> =
            button("OFF").on_press(Message::Stop).padding(40);

        let slider = Slider::new(0.0..=255.0, self.speed, Message::Speed).width(200);

        let content = Column::new()
            .align_items(Alignment::Center)
            .spacing(20)
            .push(input)
            .push(fw_button)
            .push(bw_button)
            .push(stop_button)
            .push(slider);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
