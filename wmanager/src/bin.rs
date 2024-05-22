use anyhow::Result;
use std::env;
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {

    // set display variable for testing
    env::set_var("DISPLAY", "localhost:0");

    let interactor = libwm::Interactor::new()?;
  
    let mut manager = libwm::Manager::new(interactor.get_window_on_screen(0, 1)?, 1);

    let (screen_w,screen_h) = interactor.get_screen_dim();
    let mut windows:Vec<libwm::object::Window>;

    loop {

        windows = manager.get_windows();

        for window in windows.iter_mut() {
            if !window.is_processed() {
                let (w,h) = manager.process_window_size(window.get_xid(), screen_w,screen_h);
                
                let _ = interactor.update_window_geo(window.get_xid(), 0, 0, w, h)?;
                
                window.set_process(true);

            }
        }

        manager.set_windows(windows);

        thread::sleep(Duration::from_secs(1));
    }

    Ok(())
}
