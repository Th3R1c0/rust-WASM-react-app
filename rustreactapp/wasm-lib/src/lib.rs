use wasm_bindgen::prelude::*;
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
    NoteAppRS::addnote(/* &NoteAppRS */,element);
}
#[wasm_bindgen]
pub fn delnote(element:String){
    NoteAppRS::delnote(/* &NoteAppRS */,element)
}
