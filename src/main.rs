mod lexer;
mod parser;

use iced::Alignment::Center;
use iced::widget::{Container, button, column, container, row, scrollable, text, text_input};
use iced::{Element, Length, Theme};
use lexer::Lexer;
use parser::Parser;

fn main() -> iced::Result {
    iced::application("TESTING", App::update, App::view) // You can do it without the TESTING title
        // I just do it because of my hyprland setup window rules
        .theme(App::theme)
        .window(iced::window::Settings {
            size: iced::Size {
                width: 350 as f32,
                height: 440 as f32, // Pixels are dumb
            },
            resizable: false,
            decorations: true,
            transparent: false,
            ..Default::default()
        })
        .settings(iced::Settings {
            id: Some("testing.kernelstate.org".to_string()),
            antialiasing: true,
            ..Default::default()
        })
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    AddInput(char),
    DeleteLast,
    ClearInput,
    Evaluate,
}

#[derive(Default)]
struct App {
    input_value: String,
    output_value: String,
    error_message: Option<String>,
}

impl App {
    fn update(&mut self, message: Message) {
        if self.input_value.is_empty() {
            self.output_value = "0".to_string();
            self.error_message = None;
        }
        match message {
            Message::InputChanged(value) => {
                self.input_value = value;
            }
            Message::AddInput(c) => {
                self.input_value.push(c);
            }
            Message::ClearInput => {
                self.input_value.clear();
            }
            Message::DeleteLast => {
                self.input_value.pop();
            }
            Message::Evaluate => {
                let mut lexer = Lexer::from_string(self.input_value.clone());
                lexer.lex();

                let mut parser = Parser::from_toks(lexer.toks.clone());
                match parser.eval() {
                    Ok(result) => {
                        self.output_value = result.to_string();
                    }
                    Err(e) => {
                        self.error_message = Some(format!("Error: {}", e));
                    }
                }
            }
            _ => panic!("Not implemented yet"),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let input = text_input("Try 9*3!", &self.input_value)
            .on_input(Message::InputChanged)
            .padding(10)
            .size(20);
        let result: text::Text = match &self.error_message {
            None => text(&self.output_value)
                .size(20)
                .style(text_style_secondary),
            Some(err) => text(err.clone()).size(15).style(text_style_danger),
        };

        let content = column![column![input, result].spacing(10), self.number_grid()]
            .spacing(20)
            .padding(20);

        container(scrollable(content))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn number_grid(&self) -> Container<'_, Message> {
        let nums = [
            ["7", "8", "9", "/", "C"],
            ["4", "5", "6", "*", "D"],
            ["1", "2", "3", "-", "("],
            ["0", ".", "=", "+", ")"],
        ];

        let mut grid = column![];

        for row in nums {
            let mut row_elements = row![];
            for col in row {
                let mut button_element = button(text(col).size(24).align_x(Center).align_y(Center))
                    .padding(15)
                    .width(Length::Fill);
                match col {
                    "C" => {
                        button_element = button_element
                            .on_press(Message::ClearInput)
                            .style(danger_button_style);
                    }
                    "D" => {
                        button_element = button_element
                            .on_press(Message::DeleteLast)
                            .style(main_button_style);
                    }
                    "=" => {
                        button_element = button_element
                            .on_press(Message::Evaluate)
                            .style(main_button_style);
                    }
                    _ => {
                        button_element = button_element
                            .on_press(Message::AddInput(col.to_string().chars().nth(0).unwrap()))
                            .style(normal_button_style);
                    }
                };
                row_elements = row_elements.push(button_element);
            }
            grid = grid.push(row_elements.spacing(10));
        }

        container(grid.spacing(10))
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

fn main_button_style(theme: &Theme, state: button::Status) -> button::Style {
    let palette = theme.palette();
    match state {
        button::Status::Pressed => button::Style {
            background: Some(palette.primary.into()),
            text_color: palette.text,
            border: iced::Border {
                color: palette.text,
                width: 0 as f32,
                radius: iced::border::Radius::from(8),
            },
            ..Default::default()
        },
        _ => button::Style {
            background: Some(palette.primary.into()),
            text_color: palette.text,
            border: iced::Border {
                color: palette.text,
                width: 0 as f32,
                radius: iced::border::Radius::from(8),
            },
            ..Default::default()
        },
    }
}

fn danger_button_style(theme: &Theme, state: button::Status) -> button::Style {
    let palette = theme.palette();
    match state {
        button::Status::Pressed => button::Style {
            background: Some(palette.danger.into()),
            text_color: palette.text,
            border: iced::Border {
                color: palette.text,
                width: 0 as f32,
                radius: iced::border::Radius::from(8),
            },
            ..Default::default()
        },
        _ => button::Style {
            background: Some(palette.danger.into()),
            text_color: palette.text,
            border: iced::Border {
                color: palette.text,
                width: 0 as f32,
                radius: iced::border::Radius::from(8),
            },
            ..Default::default()
        },
    }
}

fn normal_button_style(theme: &Theme, state: button::Status) -> button::Style {
    let palette = theme.palette();
    let base_light = lighten_color(palette.background.clone(), 0.1);
    let base_light2 = lighten_color(palette.background.clone(), 0.2);
    match state {
        button::Status::Pressed => button::Style {
            background: Some(base_light.into()),
            text_color: palette.text,
            border: iced::Border {
                color: palette.text,
                width: 0 as f32,
                radius: iced::border::Radius::from(8),
            },
            ..Default::default()
        },
        _ => button::Style {
            background: Some(base_light2.into()),
            text_color: palette.text,
            border: iced::Border {
                color: palette.text,
                width: 0 as f32,
                radius: iced::border::Radius::from(8),
            },
            ..Default::default()
        },
    }
}

fn text_style_secondary(theme: &Theme) -> text::Style {
    let palette = theme.palette();
    text::Style {
        color: Some(darken_color(palette.text, 0.2)),
        ..Default::default()
    }
}

fn text_style_danger(theme: &Theme) -> text::Style {
    let palette = theme.palette();
    text::Style {
        color: Some(palette.danger),
        ..Default::default()
    }
}

fn lighten_color(color: iced::Color, amount: f32) -> iced::Color {
    iced::Color {
        r: (color.r + amount).min(1.0),
        g: (color.g + amount).min(1.0),
        b: (color.b + amount).min(1.0),
        a: color.a,
    }
}

fn darken_color(color: iced::Color, amount: f32) -> iced::Color {
    iced::Color {
        r: (color.r - amount).max(0.0),
        g: (color.g - amount).max(0.0),
        b: (color.b - amount).max(0.0),
        a: color.a,
    }
}
