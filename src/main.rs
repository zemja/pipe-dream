mod output;
mod shell;
mod ui;

fn main() {
    relm4::RelmApp::new("org.zemja.pipe-dream").run::<ui::pipe_dream::Model>(());
}