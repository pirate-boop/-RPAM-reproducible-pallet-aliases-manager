mod action;
mod scanner;

use std::path::Path;

use anyhow::Result;

fn main() -> Result<()> {
    let actions = scanner::scan_actions(
        Path::new("./examples")
    )?;

    for action in actions {
        println!("{:#?}", action);
    }

    Ok(())
}
