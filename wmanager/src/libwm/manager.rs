
use super::{object,xcall};

use xcb::{x, Event, Xid};
use xcb::x::Event as XEvent;
use anyhow::Result;
use std::collections::HashMap;

pub struct Manager {

    curr_workspace:     String,
    workspaces:         HashMap<String,object::Workspace>,
    screen_size:        object::Size        

}

impl Manager {
    
    pub fn new(w:u16,h:u16) -> Self {

        Self {
            curr_workspace: "1".to_string(),
            workspaces:     gen_workspace(),
            screen_size:    object::Size { w, h }
        }

    }

    pub fn process_event(&mut self,connector: &xcb::Connection) -> Result<()>{

        let mut update_layout = false;
    
        if let Event::X(xevent) = connector.wait_for_event()? {

            match xevent {

                // NOTIFY

                XEvent::CreateNotify(notify) => {
                     
                    println!(
                        "new window created with title : {}",
                        xcall::get_string_window_property(connector, &notify.window(), x::ATOM_WM_NAME)?
                    );

                    // add window to a workspace (current for now)
                    // TODO: config file with application map to specific workspace
                    self.workspaces.get_mut(&self.curr_workspace).unwrap().push_window(notify.window());
                    
                    update_layout = true;
                },
                
                XEvent::MapNotify(notify) => println!("window {} have been mapped",notify.window().resource_id()),

                XEvent::DestroyNotify(notify) => {
                    self.workspaces.get_mut(&self.curr_workspace).unwrap().remove_window(&notify.window());
                },

                // REQUEST
                XEvent::ConfigureRequest(req) => {
                    println!("need to be configure hahahahha");
                },

                XEvent::MapRequest(req) => {
                    
                    xcall::map_window(connector, &req.window())?;

                },

                _ => println!("event not process: {:?}",xevent)

                
            }

        } else {
            unimplemented!();
        }

        // update the current workspace layout
        if update_layout {
            self.update_workspace(connector)?;
        }

        Ok(())

    }


    fn update_workspace(&self,connector: &xcb::Connection) -> Result<()> {

        let workspace = self.workspaces.get(&self.curr_workspace).unwrap();

        let mut x:i32 = 0;
        let mut y:i32 = 0;
        let mut w:u32 = 0;
        let h = self.screen_size.h as u32 / workspace.get_row_count() as u32;

        for (r,row) in workspace.get_layout().iter().enumerate() {
            
            y = (r as u32 * h) as i32;

            w = self.screen_size.w as u32 / row.len() as u32;

            for (c,window) in row.iter().enumerate() {

                x = (c as u32 * w) as i32;

                xcall::configure_window(&connector, window, x,y,w,h)?;

            }

        }

        Ok(())
    }

}

// TMP: generate default workspace
fn gen_workspace() -> HashMap<String,object::Workspace> {

    let mut workspaces:HashMap<String,object::Workspace> = HashMap::new();

    for i in 0..10 {
        workspaces.insert(format!("{}",i), object::Workspace::new());
    }

    workspaces
}