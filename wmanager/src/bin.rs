use anyhow::Result;
use std::env;

fn main() -> Result<()> {

    // set display variable for testing
    env::set_var("DISPLAY", "localhost:0");

    let interactor = libwm::Interactor::new()?;

    for win in interactor.get_window_on_screen(0)?.iter() {
        println!("{:?}",win);
    }

    Ok(())
}
