use wasm_bindgen::prelude::*;
struct NoteAppRS;
impl NoteAppRS{
    fn addnote(mut notelist: Vec<String>,element: String){
        notelist.push(element);
        println!("{:?}",notelist);
    }
    fn delnote(mut notelist: Vec<String>,element:String){
        let index = note_list
        .iter()
        .position(|&r| r == element)
        .unwrap();
        notelist.remove(index);
    }
    fn get_values(notelist: Vec<String>) -> Vec<String>{
      notelist
    }
}
fn main(){
    let note_list = Vec::new();
    let main = "he"; // this is an example ("he", will be whatever the user puts in as a note)
    NoteAppRS::addnote(note_list,main.to_string());
}

#[wasm_bindgen]
pub fn addnote(element:String){
    NoteAppRS::addnote(note_list,element);
}
#[wasm_bindgen]
pub fn delnote(element:String){
    NoteAppRs::delnote(note_list,element)
}
#[wasm_bindgen]
pub fn get_values(){
    get_values(note_list)
}
