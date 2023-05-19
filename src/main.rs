#![windows_subsystem = "windows"]

use std::ffi::OsStr;
use std::path::PathBuf;
use std::{fs, io};

use iced::alignment::{Horizontal, Vertical};
use iced::widget::{text, text_input, Column, Container, Space, Text};
use iced::{executor, window, Application, Command, Element, Length, Settings, Theme};
use iced_native::widget::container;
use iced_native::{row, Alignment, Event, Subscription};
use sha2::{Digest, Sha256};

mod style;

struct FileHash {
    file_name: String,
    file_path: PathBuf,
    hash: String,
    user_hash: String,
    hash_matches: bool,
}

#[derive(Debug, Clone)]
enum Message {
    DroppedFile(Event),
    InputChanged(String),
}

impl Application for FileHash {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (FileHash, Command<Message>) {
        (
            FileHash {
                file_name: String::new(),
                file_path: PathBuf::new(),
                hash: String::new(),
                user_hash: String::new(),
                hash_matches: false,
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

                            if let Some(value) = self.file_path.file_name() {
                                if let Some(name) = value.to_str() {
                                    self.file_name = String::from(name)
                                }
                            }

                            if self.hash == self.user_hash {
                                self.hash_matches = true
                            } else {
                                self.hash_matches = false
                            }
                        }
                        Err(error) => {
                            println!("error opening file: {error}");
                        }
                    }
                } else {
                }
                Command::none()
            }
            Message::InputChanged(input) => {
                self.user_hash = input;

                if self.hash == self.user_hash {
                    self.hash_matches = true
                } else {
                    self.hash_matches = false
                }

                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let file_drop_area = Container::new(row![text("Drop File Here").size(40.0)])
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .height(100.0)
            .padding(10)
            .style(style::DropFileContainer);

        let path_hash_text = Container::new(
            Column::new()
                .push(text(&self.file_name).size(20.0))
                .push(text(&self.hash))
                .align_items(Alignment::Center)
                .spacing(10.0)
                .height(50.0)
                .width(625),
        )
        .padding(15.0);

        let compare_text = if self.hash_matches {
            Text::new("Both hashes match!").height(20.0)
        } else {
            Text::new("Hashes do not match!").height(20.0)
        };

        Column::new()
            .align_items(Alignment::Center)
            .push(Space::new(0, 50))
            .push(file_drop_area)
            .push(path_hash_text)
            .height(Length::Fill)
            .width(Length::Fill)
            .push(
                container(
                    text_input("Input hash you want to compare..", &self.user_hash)
                        .on_input(Message::InputChanged),
                )
                .align_x(Horizontal::Center)
                .align_y(Vertical::Bottom)
                .width(625)
                .padding(10.0),
            )
            .push(container(compare_text).padding(10.0))
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
            size: (650, 325),
            resizable: true,
            decorations: true,

            ..Default::default()
        },
        ..Default::default()
    };
    FileHash::run(settings).unwrap();
}
