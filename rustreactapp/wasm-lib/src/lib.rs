use wasm_bindgen::prelude::*;
pub fn init(){
    let note_list = Vec::<String>::new();
}
#[wasm_bindgen]
pub fn addnote(element: String){
    note_list.push(element);
    println!("{:?}",note_list);
}
#[wasm_bindgen]
pub fn delnote(element:String){
    let index = note_list
    .iter()
    .position(|&r| r == element)
    .unwrap();
    note_list.remove(index);
}
#[wasm_bindgen]
pub fn get_values(){
    for note in note_list.iter(){
        return note.to_string();
    };
}
