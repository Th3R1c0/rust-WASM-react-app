use wasm_bindgen::prelude::*;
#[wasm_bindgen]
pub fn addnote(mut notelist: Vec<String>,element: String){
    notelist.push(element);
    println!("{:?}",notelist);
  }
fn main(){
    let note_list = Vec::new();
    let main = "he"; // this is an example ("he", will be whatever the user puts in as a note)
    addnote(note_list,main.to_string());
  }
