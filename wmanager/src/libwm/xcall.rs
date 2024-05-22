use xcb::x::{self, Str};
use super::object;

#[derive(thiserror::Error,Debug)]
pub enum XError {
    // tmp
    #[error(transparent)]
    XServerError(#[from] xcb::Error),
    #[error(transparent)]
    Connection(#[from] xcb::ConnError),
    #[error(transparent)]
    Protocol(#[from] xcb::ProtocolError),
    #[error("No screen with {0} exist")]
    BadScreenNum(usize)

}




pub struct Interactor {
    conn: xcb::Connection,
} 

impl Interactor {
    
    pub fn new() -> Result<Self,XError> {

        let (conn, _) = xcb::Connection::connect(None)?;
        Ok( Self { conn })

    }

    pub fn get_root_screen(&self,screen_num:usize) -> Result<Box<&x::Screen>,XError> {

        let setup = self.conn.get_setup();

        if let Some(screen) = setup.roots().nth(screen_num) {
            return Ok(Box::new(screen));
        }

        Err(XError::BadScreenNum(screen_num))

    }


    pub fn get_window_on_screen(&self, screen_num:usize) -> Result<Vec<object::Window>,XError> {

        let screen = self.get_root_screen(screen_num)?;
        let mut windows: Vec<object::Window> = Vec::new();

        let cookie = self.conn.send_request(&x::QueryTree { window: screen.root()});
        
        for child in self.conn.wait_for_reply(cookie)?.children() {

            windows.push(
                object::Window {
                    class: self.get_window_property(child, x::ATOM_WM_CLASS)? 
                }
            )

        } 


        Ok(windows)

    }

    fn get_window_property(&self, window:&x::Window, property:x::Atom) -> Result<String,XError> {

        let cookie = self.conn.send_request(&x::GetProperty {
            delete: false,
            window: *window,
            property,
            r#type: x::ATOM_STRING,
            long_offset: 0,
            long_length: 1000

        });
        
        let reply = self.conn.wait_for_reply(cookie)?;
        
        Ok(std::str::from_utf8(reply.value()).unwrap().to_string())

    }

}