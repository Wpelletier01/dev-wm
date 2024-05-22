
use std::ops::DivAssign;

use super::object;

pub struct Manager {

    workspace:  u8,
    windows:    Vec<object::Window>

}

impl Manager {
    
    pub fn new(windows:Vec<object::Window>,workspace:u8) -> Self {
        Self {
            workspace,
            windows
        }
    }
    
    pub fn get_windows(&self) -> Vec<object::Window> { self.windows.to_vec() }
    pub fn set_windows(&mut self,windows:Vec<object::Window>) { self.windows = windows; }
    pub fn process_window_size(&mut self,idx:u32,screen_w:u16,screen_h:u16) -> (u16,u16) {

        let nb_win = self.get_nb_win_in_workspace();

        if nb_win == 1 { return (screen_w,screen_h); }


        ((screen_w/nb_win as u16) as u16,screen_h)


    }    


    fn get_nb_win_in_workspace(&self) -> usize {
        self.windows.iter().filter(|w| w.get_workspace() == self.workspace ).count()
    }



}

