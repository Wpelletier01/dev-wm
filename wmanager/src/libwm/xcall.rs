use xcb::{x, Xid};
use super::object;

const DEFAULT_SCREEN: i32 = 0;


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
    BadScreenNum(usize),
    #[error("No window with the id '{0}' exist")]
    BadWindowId(u32)

}


pub fn get_string_window_property(connector: &xcb::Connection, window:&x::Window, property:x::Atom) -> Result<String,XError> {

    let cookie = connector.send_request(&x::GetProperty {
        delete: false,
        window: *window,
        property,
        r#type: x::ATOM_STRING,
        long_offset: 0,
        long_length: 1000

    });
    
    let reply = connector.wait_for_reply(cookie)?;
                
    Ok(std::str::from_utf8(reply.value()).unwrap().to_string())


}

pub fn change_win_attributes(connector: &xcb::Connection, window:&x::Window, values: &[x::Cw]) -> Result<(),XError> {

    let cookie = connector.send_request(&x::ChangeWindowAttributes {
        window: *window,
        value_list: values
    });

    connector.flush()?;

    Ok(())

}

pub fn map_window(connector: &xcb::Connection, window:&x::Window) -> Result<(),XError> {

    let cookie = connector.send_request(&x::MapWindow  { window: *window });

    connector.flush()?;

    Ok(())

}


pub fn set_root_as_wmanager(connector: &xcb::Connection, window:&x::Window) -> Result<(),XError> {

    change_win_attributes(
        connector, 
        window, 
        &[x::Cw::EventMask(
            x::EventMask::SUBSTRUCTURE_NOTIFY | x::EventMask::SUBSTRUCTURE_REDIRECT |
            x::EventMask::KEYMAP_STATE | x::EventMask::KEY_PRESS | x::EventMask::KEY_RELEASE
            )
        ]
    )?; 

    Ok(())
}

pub fn configure_window(
    connector: &xcb::Connection, 
    window:&x::Window, 
    x:i32, 
    y:i32, 
    w:u32, 
    h:u32 ) -> Result<(),XError> {


    println!("x: {x} y: {y} w: {w} h: {h}");

    let cookie = connector.send_request(&x::ConfigureWindow {
        window: *window,
        value_list: &[ 
            x::ConfigWindow::X(x),
            x::ConfigWindow::Y(y),
            x::ConfigWindow::Width(w),
            x::ConfigWindow::Height(h)
        ]
    });

    connector.flush()?;

    Ok(())
}


//pub struct Interactor {
//    conn: xcb::Connection,
//} 
//
//impl Interactor {
//    
//    pub fn new() -> Result<Self,XError> {
//
//        let (conn, _) = xcb::Connection::connect(None)?;
//        Ok( Self { conn })
//
//    }
//
//    pub fn get_root_screen(&self,screen_num:usize) -> Result<Box<&x::Screen>,XError> {
//
//        let setup = self.conn.get_setup();
//
//        if let Some(screen) = setup.roots().nth(screen_num) {
//            return Ok(Box::new(screen));
//        }
//
//        Err(XError::BadScreenNum(screen_num))
//
//    }
//
//    pub fn get_screen_dim(&self) -> (u16,u16) { 
//        let screen = self.get_root_screen(0).unwrap();
//
//        (screen.width_in_pixels(),screen.height_in_pixels())
//
//    }
//
//    pub fn get_last_event(&self) -> Result<xcb::Event,XError> { Ok(self.conn.wait_for_event()?) }
//
//    pub fn update_window_geo(&self,idx:u32, x:i16,y:i16,w:u16,h:u16) -> Result<(),XError> {
//        
//        
//        let screen = self.get_root_screen(0)?;
//
//        let cookie = self.conn.send_request(&x::QueryTree { window: screen.root()});
//
//
//        for child in self.conn.wait_for_reply(cookie)?.children() {
//            
//            if child.resource_id() == idx {
//
//                let change_cookie = self.conn.send_request(&x::ConfigureWindow {
//                    window: *child,
//                    value_list: &[
//                        x::ConfigWindow::X(x.into()),
//                        x::ConfigWindow::Y(y.into()),
//                        x::ConfigWindow::Width(w.into()),
//                        x::ConfigWindow::Height(h.into()),
//                    ],
//                });
//
//                self.conn.flush()?;
//
//
//                return Ok(());                
//
//            }
//
//        }
//
//        Err(XError::BadWindowId(idx))
//
//
//    }
//
//    pub fn get_window_on_screen(&self, screen_num:usize, workspace:u8) -> Result<Vec<object::Window>,XError> {
//
//        let screen = self.get_root_screen(screen_num)?;
//
//        let mut windows: Vec<object::Window> = Vec::new();
//
//        let mut title:String;
//        let mut class:String;
//
//        let cookie = self.conn.send_request(&x::QueryTree { window: screen.root()});
//        
//        for child in self.conn.wait_for_reply(cookie)?.children() {
//            
//            title = self.get_string_window_property(child, x::ATOM_WM_NAME)?;
//            class = self.get_string_window_property(child, x::ATOM_WM_CLASS)?;
//
//            // tmp filter
//            if !class.is_empty() && !title.is_empty() {
//                windows.push(
//                    object::Window::new( 
//                        child.resource_id(),
//                        workspace,
//                        self.get_string_window_property(child, x::ATOM_WM_NAME)?
//                    )
//                );
//
//            }
//
//        } 
//
//
//        Ok(windows)
//
//    }
//
//    fn get_string_window_property(&self, window:&x::Window, property:x::Atom) -> Result<String,XError> {
//
//    
//        let cookie = self.conn.send_request(&x::GetProperty {
//            delete: false,
//            window: *window,
//            property,
//            r#type: x::ATOM_STRING,
//            long_offset: 0,
//            long_length: 1000
//
//        });
//        
//        let reply = self.conn.wait_for_reply(cookie)?;
//                    
//        Ok(std::str::from_utf8(reply.value()).unwrap().to_string())
//
//
//    }
//
//    
//
//
//}

