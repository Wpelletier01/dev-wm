use anyhow::Result;
use libwm::xcall;
use std::env;
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {

    // set display variable for testing
    env::set_var("DISPLAY", "localhost:0");
    // x11 important stuff
    let (connector,_) =  xcb::Connection::connect(None)?;
    let setup = connector.get_setup();
    // default screen for now
    let screen = setup.roots().nth(0).expect("No screen with number 0");
    let root = screen.root();
    // set root window as a window manager
    xcall::set_root_as_wmanager(&connector, &root)?;


    
    let mut manager = libwm::Manager::new(screen.width_in_pixels(),screen.height_in_pixels());


    loop {
        
        manager.process_event(&connector)?;

    }

    Ok(())
}
