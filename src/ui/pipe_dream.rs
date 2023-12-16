use relm4::prelude::*;
use relm4::gtk::prelude::*;
use crate::output::Output;
use crate::shell;

pub struct Model {
    last_output: shell::Result<Output>,
    buffer: gtk::TextBuffer,
    shell: shell::Result<shell::Nushell>,
}

#[derive(Debug)]
pub enum Input {
    PromptSubmitted(String),
}

#[relm4::component(pub)]
impl SimpleComponent for Model {
    type Init = ();
    type Input = Input;
    type Output = ();

    fn init(_init: (), _root: &Self::Root, sender: ComponentSender<Model>)
        -> ComponentParts<Model> {
        let model = Model {
            last_output: Ok(Output::Empty),
            buffer: gtk::TextBuffer::new(None),
            shell: shell::Nushell::new(),
        };

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    view! {
        gtk::Window {
            set_title: Some("Pipe Dream"),
            set_default_size: (640, 480),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                gtk::ScrolledWindow {
                    gtk::TextView {
                        set_buffer: Some(&model.buffer),
                        set_vexpand: true,
                    }
                },

                gtk::Entry {
                    set_placeholder_text: Some(">"),
                    connect_activate[sender] => move |entry| sender.input_sender()
                        .emit(Input::PromptSubmitted(entry.text().to_string())),
                }
            }
        }
    }

    fn update(&mut self, message: Input, _sender: ComponentSender<Model>) {
        match message {
            Input::PromptSubmitted(prompt) => {
                if let Ok(shell) = self.shell.as_mut() {
                    self.last_output = shell.evaluate(&prompt).map(|output| output.into());
                    self.buffer.set_text(&format!("{:#?}", self.last_output));
                }
            }
        }
    }
}
