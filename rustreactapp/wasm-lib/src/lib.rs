use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn addnote(mut notelist: Vec<String>,element: String){
    notelist.push(element);
    println!("{:?}",notelist);
}
#[wasm_bindgen]
pub fn delnote(mut notelist: Vec<String>,element:String){
    let index = notelist
    .iter()
    .position(|&r| r == element)
    .unwrap();
    notelist.remove(index);
}
#[wasm_bindgen]
pub fn get_values(mut notelist: Vec<String>) -> Vec<String>{
    notelist
}
#[wasm_bindgen]
pub fn init(){ //Run this function at the start of the javascript program
    let note_list = Vec::<String>::new();
  }
