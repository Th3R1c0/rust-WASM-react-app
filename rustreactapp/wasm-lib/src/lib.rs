use wasm_bindgen::prelude::*;
use sqlite::Error as sqErr;

pub struct Note{
    note:String,
}

#[derive(Debug)]
pub enum NoteErr{
    DbErr(sqErr),
}

impl From<sqlErr> for NoteErr{
    fn from(s:sqErr)->Self{
        NoteErr::DbErr(s);
    }
}

impl Note{
    pub fn add(&self,note:&str) -> Result<(),NoteErr>{
        let connection = sqlite::open(&self.note)?;
        let mut db = connection.
    }
}


/*
struct NoteAppRS{
    note_list_rs: Vec<String>,
}
impl NoteAppRS{
    fn addnote(&self,element: String){
        self.note_list_rs.push(element);
    }
    fn delnote(&self,element:String){
        let index = self.note_list_rs
        .iter()
        .position(|&r| r == element)
        .unwrap();
        self.note_list_rs.remove(index);
    }
    fn get_values(&self) -> Vec<String>{
        self.note_list_rs
    }
}


#[wasm_bindgen]
pub fn addnote(element:String){
    NoteAppRS::addnote(element);
}
#[wasm_bindgen]
pub fn delnote(element:String){
    NoteAppRS::delnote(element)
}
*/
