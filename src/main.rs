mod output;
mod shell;
mod theme;

use s_macro::s;
use iced::widget::{Column, Container, Row, Scrollable, Text, TextInput};
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
    type Flags = ();
    type Message = Message;
    type Theme = theme::Theme;

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
            Ok(output) => output.into(),
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

impl<'a> From<&'a Output> for Element<'a> {
    fn from(value: &'a Output) -> Element {
        match value {
            Output::Empty => Text::new(s!("Nothing"))
                .style(Style::Shadow)
                .width(Length::Fill)
                .into(),

            Output::Value(value) => value.into(),

            Output::Table(table) => {
                let mut rows = vec![
                    Row::with_children(table.header_row.iter()
                        .map(|column| Text::new(column)
                            .style(Style::Emphasis)
                            .horizontal_alignment(Horizontal::Center)
                            .width(Length::Fill)
                            .into())
                        .collect())
                ];

                for row in table.rows.iter() {
                    rows.push(Row::with_children(row.iter().map(Element::from).collect()));
                }

                Column::with_children(rows.into_iter().map(Element::from).collect()).into()
            }

            Output::List(values) => Column::with_children(values.iter()
                    .map(Element::from).collect())
                .into(),

            Output::Raw(Err(err)) => Text::new(s!("Error reading raw stream: {err}"))
                .style(Style::Error)
                .into(),

            Output::Raw(Ok(bytes)) => Text::new(String::from_utf8_lossy(bytes).to_string()).into(),
        }
    }
}

impl<'a> From<&output::Value> for Element<'a> {
    fn from(value: &output::Value) -> Element<'a> {
        match value {
            output::Value(nu_protocol::Value::Nothing { .. })
                => Text::new(s!("Nothing")).style(Style::Shadow),

            value => Text::new(value.0.into_string(", ", &nu_protocol::Config::default()))
        }
            .width(Length::Fill)
            .into()
    }
}