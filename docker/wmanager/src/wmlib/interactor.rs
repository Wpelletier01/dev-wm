

use xcb::Connection;


pub struct Interactor {
    
    connector: Connection

}


impl Interactor {

    pub fn init() -> xcb::Result<Self> {

        let (conn,screen_n) = xcb::Connection::connect(None)?;
        

        Ok(Self { 
           connector: conn
        })
    } 

}








