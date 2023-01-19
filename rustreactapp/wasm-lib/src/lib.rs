use wasm_bindgen::prelude::*;
struct NoteAppRS{
    note_list_rs: Vec<String>,
}
impl NoteAppRS{
    fn addnote(element: String){
        self.note_list_rs.push(element);
    }
    fn delnote(element:String){
        let index = note_list
        .iter()
        .position(|&r| r == element)
        .unwrap();
        self.note_list_rs.remove(index);
    }
    fn get_values() -> Vec<String>{
        self.note_list_rs
    }
}
fn main(){
    let mut note_list = Vec::<String>::new();
    NoteAppRS::addnote(note_list,main.to_string());
}

#[wasm_bindgen]
pub fn addnote(element:String){
    NoteAppRS::addnote(element);
}
#[wasm_bindgen]
pub fn delnote(element:String){
    NoteAppRS::delnote(element)
}
