use xcb::Xid;


type Layout = Vec<Vec<xcb::x::Window>>;


pub struct Workspace {

    active: bool,
    layout: Layout

}

impl Workspace {
    
    pub fn new() -> Self {
        
        Self {
            active: false,
            layout: vec![vec![]]
        }
    }

    pub fn push_window(&mut self, window: xcb::x::Window) { 
        self.layout[0].push(window); 
    }

    pub fn remove_window(&mut self, window: &xcb::x::Window) {

        
       for row in self.layout.iter_mut() {
            
            if let Some(index) = row.iter().position(|w| w.resource_id() == window.resource_id() ) {
                println!("remove {:?}",window);

                let _ = row.remove(index);                
                break;
            }

        }

    }

    pub fn get_layout(&self) -> &Layout { &self.layout }
    pub fn get_row_count(&self) -> usize { self.layout.len() }
}



pub struct Size { 
    pub w:  u16,
    pub h:  u16
}

