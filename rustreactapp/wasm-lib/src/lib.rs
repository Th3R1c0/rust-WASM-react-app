use wasm_bindgen::prelude::*;
#[wasm_bindgen]
struct NoteApp;
impl NoteApp{
    pub fn addnote(mut notelist: Vec<String>,element: String){
        notelist.push(element);
        println!("{:?}",notelist);
    }
    pub fn delnote(mut notelist: Vec<String>,element:String){
      let index = notelist.iter().position(element).unwrap();
      notelist.remove(index);
    }
    pub fn get_values(mut notelist: Vec<String>){
      return notelist;
    }
}
fn main(){
    let note_list = Vec::new();
    let main = "he"; // this is an example ("he", will be whatever the user puts in as a note)
    //so you will get the note feild and then pass in a value to this function
    //than you can do a for loop through "notelist"
    // accessing the values of "notelist" by the "get_values" function
    addnote(note_list,main.to_string());
  }
