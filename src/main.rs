#![windows_subsystem = "windows"]

use std::path::PathBuf;
use std::{fs, io};

use iced::alignment::{Horizontal, Vertical};
use iced::widget::{text, Column};
use iced::{executor, window, Application, Command, Element, Length, Settings, Theme};
use iced_native::widget::container;
use iced_native::{row, Alignment, Event, Subscription};
use sha2::{Digest, Sha256};

struct FileHash {
    file_path: PathBuf,
    hash: String,
}

#[derive(Debug, Clone)]
enum Message {
    DroppedFile(Event),
}

impl Application for FileHash {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (FileHash, Command<Message>) {
        (
            FileHash {
                file_path: PathBuf::new(),
                hash: String::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Easy File Hash")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::DroppedFile(event) => {
                if let Event::Window(window::Event::FileDropped(file)) = event {
                    self.file_path = file;
                    let mut hash_algo = Sha256::new();

                    match fs::File::open(&self.file_path) {
                        Ok(mut file) => {
                            io::copy(&mut file, &mut hash_algo).unwrap();
                            let finalized = hash_algo.finalize();
                            self.hash = hex::encode(finalized);
                        }
                        Err(error) => {
                            println!("error opening file: {error}");
                        }
                    }
                } else {
                }
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        Column::new()
            .align_items(Alignment::Center)
            .push(
                container(row![text("Drop File Here").size(40.0)])
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Center)
                    .height(100.0),
            )
            .push(
                container(row![
                    text(self.file_path.to_str().unwrap().to_string()).size(20.0)
                ])
                .align_x(Horizontal::Center)
                .align_y(Vertical::Bottom)
                .width(Length::Shrink)
                .padding(10.0),
            )
            .push(container(text(&self.hash)).padding(10.0))
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn subscription(&self) -> Subscription<Message> {
        iced_native::subscription::events().map(Message::DroppedFile)
    }
}

fn main() {
    println!("Easy File Hash");
    let settings = Settings {
        window: window::Settings {
            size: (650, 200),
            resizable: true,
            decorations: true,

            ..Default::default()
        },
        ..Default::default()
    };
    FileHash::run(settings).unwrap();
}
