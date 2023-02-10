mod output;
mod shell;
mod theme;

use s_macro::s;
use iced::widget::{Column, Container, Row, Scrollable, Space, Text, TextInput};
use iced::{Application, Command, Length, Settings};
use iced_graphics::alignment::Horizontal;
use crate::output::Output;
use crate::theme::Style;

pub fn main() -> iced::Result {
    PipeDream::run(Settings {
        default_font: Some(include_bytes!("../res/inconsolata.ttf")),
        default_text_size: 16,
        ..Default::default()
    })
}

struct PipeDream {
    last_output: shell::Result<Output>,
    prompt: String,
    shell: shell::Result<shell::Nushell>,
}

#[derive(Debug, Clone)]
enum Message {
    PromptChanged(String),
    PromptSubmitted,
}

type Element<'a>
    = iced::Element<'a, Message, iced_graphics::Renderer<iced_wgpu::Backend, theme::Theme>>;

impl Application for PipeDream {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = theme::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (PipeDream, Command<Message>) {
        let pipe_dream = PipeDream {
            last_output: Ok(Output::Empty),
            prompt: s!(),
            shell: shell::Nushell::new(),
        };

        (pipe_dream, Command::none())
    }

    fn title(&self) -> String {
        s!("Pipe Dream")
    }

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            Message::PromptChanged(prompt) => self.prompt = prompt,
            Message::PromptSubmitted => {
                if let Ok(shell) = self.shell.as_mut() {
                    self.last_output = shell.evaluate(&self.prompt).map(|output| output.into());
                }
            },
        }

        Command::none()
    }

    fn view(&self) -> Element {
        if let Err(err) = self.shell.as_ref() {
            return Text::new(s!("Error initialising shell: {err}")).style(Style::Error).into();
        }

        let body: Element = match self.last_output.as_ref() {
            Ok(output) => output.clone().into(),
            Err(err) => Text::new(s!("Error: {err}")).style(Style::Error).into(),
        };

        let body = Container::new(body).padding(5);

        let prompt = TextInput::new("> ", &self.prompt, Message::PromptChanged)
            .on_submit(Message::PromptSubmitted);

        Column::new()
            .push(Scrollable::new(Container::new(body).width(Length::Fill)).height(Length::Fill))
            .push(prompt)
            .spacing(5)
            .into()
    }
}

impl<'a> From<Output> for Element<'a> {
    fn from(value: Output) -> Element<'a> {
        match value {
            Output::Empty => Text::new(s!("Nothing"))
                .style(Style::Shadow)
                .width(Length::Fill)
                .into(),

            Output::Value(value) => value.into(),

            Output::Table(table) => {
                let mut rows = vec![
                    Row::with_children(table.header_row.into_iter()
                        .map(|column| Text::new(column)
                            .style(Style::Emphasis)
                            .horizontal_alignment(Horizontal::Center)
                            .width(Length::Fill)
                            .into())
                        .collect())
                ];

                for row in table.rows.into_iter() {
                    rows.push(Row::with_children(row.into_iter().map(Element::from).collect()));
                }

                Column::with_children(rows.into_iter().map(Element::from).collect()).into()
            }

            Output::List(values)
                => Column::with_children(values.into_iter().map(Element::from).collect()).into(),

            Output::Raw(Err(err)) => Text::new(s!("Error reading raw stream: {err}"))
                .style(Style::Error)
                .into(),

            Output::Raw(Ok(bytes))
                => Text::new(String::from_utf8_lossy(bytes.as_slice()).to_string()).into(),
        }
    }
}

fn make_column<'a>(values: impl Iterator<Item = nu_protocol::Value>) -> Element<'a> {
    Column::with_children(values.map(|val| match val {
        nu_protocol::Value::Record { cols, .. }
            => Text::new(s!("Record {} rows", cols.len())).into(),
        nu_protocol::Value::List { vals, .. }
            => Text::new(s!("List {} rows", vals.len())).into(),
        val => Element::from(output::Value(val)),
    }).collect())
        .into()
}

impl<'a> From<output::Value> for Element<'a> {
    fn from(value: output::Value) -> Element<'a> {
        match value {
            output::Value(nu_protocol::Value::Nothing { .. })
                => Text::new(s!("Nothing")).style(Style::Shadow).width(Length::Fill).into(),

            output::Value(nu_protocol::Value::Error { error })
                => Text::new(error.to_string()).style(Style::Error).width(Length::Fill).into(),

            output::Value(nu_protocol::Value::Record { cols, vals, .. }) => {
                Row::new()
                    .push(Column::with_children(cols.into_iter()
                        .map(|col| Text::new(col)
                            .style(Style::Emphasis)
                            .horizontal_alignment(Horizontal::Right)
                            .into())
                        .collect()))
                    .push(Space::new(Length::Units(10), Length::Shrink))
                    .push(make_column(vals.into_iter()))
                    .into()
            }

            output::Value(nu_protocol::Value::List { vals, .. }) => make_column(vals.into_iter()),

            value => Text::new(value.0.into_string(", ", &nu_protocol::Config::default()))
                .width(Length::Fill).into(),
        }
    }
}