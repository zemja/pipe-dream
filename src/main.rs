use std::process::ExitCode;

mod nushell;
mod output;

slint::include_modules!();

// TODO Use res/inconsolata.ttf again.

fn main() -> ExitCode {
    if let Err(err) = run() {
        eprintln!("error: {err}");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mw = MainWindow::new()?;
    let mut nu = nushell::Nushell::new()?;

    let weak = mw.as_weak();
    mw.on_accepted(move |text| {
        let Some(mw) = weak.upgrade() else { return };
        mw.set_prompt("".into());
        mw.set_text(format!("{:#?}", nu.evaluate(&text).map(output::Output::from)).into());
    });

    mw.run()?;
    Ok(())
}