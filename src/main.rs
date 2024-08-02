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

        match nu.evaluate(&text).map(output::Output::from) {
            Ok(output::Output::Empty) => mw.set_type(Type::Empty),

            Ok(output::Output::Value(value)) => {
                mw.set_type(Type::Value);
                mw.set_value(format!("{value:?}").into());
            }

            Ok(output::Output::List(list)) => {
                let list: Vec<_> = list.into_iter()
                    .map(|value| format!("{value:?}").into())
                    .collect();
                mw.set_type(Type::List);
                mw.set_list(slint::ModelRc::new(slint::VecModel::from(list)));
            }

            Ok(output::Output::Table(table)) => {
                let rows = table.rows.into_iter()
                    .map(|row| row.into_iter().map(|row| format!("{row:?}")).collect());
                let table: Vec<Vec<String>> = [table.header_row].into_iter().chain(rows).collect();
                let rows: Vec<slint::ModelRc<slint::SharedString>> = table.into_iter()
                    .map(|row| {
                        let row: Vec<slint::SharedString> = row.into_iter()
                            .map(slint::SharedString::from)
                            .collect();
                        slint::ModelRc::new(slint::VecModel::from(row))
                    })
                    .collect();
                let table = slint::ModelRc::new(slint::VecModel::from(rows));
                mw.set_type(Type::Table);
                mw.set_table(table);
            }

            Ok(output::Output::Raw(Ok(raw))) => {
                mw.set_type(Type::Raw);
                mw.set_raw(slint::SharedString::from(String::from_utf8_lossy(&raw).to_string()));
            }

            Ok(output::Output::Raw(Err(err))) => {
                mw.set_type(Type::Error);
                mw.set_error(err.to_string().into());
            }

            Err(err) => {
                mw.set_type(Type::Error);
                mw.set_error(err.to_string().into());
            }
        }

        mw.set_prompt("".into());
    });

    mw.run()?;
    Ok(())
}