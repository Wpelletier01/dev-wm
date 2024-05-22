



use std::env;

fn main() -> xcb::Result<()> {
    
    env::set_var("DISPLAY","xserver.lan.com:0");

    let interactor = wmlib::Interactor::init()?;

    Ok(())
}
