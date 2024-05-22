
#[derive(Debug,Clone)]
pub struct Window {
    id:         u32,
    title:      String,
    workspace:  u8,
    processed:  bool
}

impl Window {


    pub fn new(id:u32, workspace:u8, title:String) -> Self {
        Self {
            id,
            workspace,
            title,
            processed: false
        }
    }

    pub fn is_processed(&self) -> bool { self.processed }
    pub fn get_workspace(&self) -> u8 { self.workspace }
    pub fn get_title(&self) -> &str { &self.title }
    pub fn get_xid(&self) -> u32 { self.id }

    pub fn set_process(&mut self,p:bool) { self.processed = true }

}


