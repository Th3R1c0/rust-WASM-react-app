#![allow(unused)]
use sqlx::postgres::{PgPoolOptions,PgRow};
use sqlx::{FromRow,Row};

#[derive(Debug,FromRow)]
struct Note{
    id: i64,
    content: String,
}
struct SqlStruct;
impl SqlStruct{
    #[tokio::main]
    pub async fn main() -> Result<(), sqlx::Error>{
        let pool = PgPoolOptions::new().max_connections(5)
        .connect("postgres://postgres:welcom@localhost/postgres")
        .await?;
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS notes(
                id bigserial,
                content text
            );
            "#,
        ).execute(&pool)
        .await?;
        };
    pub fn inserting_note(){
        let row: (i64,) = sqlx::query_as("insert into notes (content) values ($1) returning id")
            .bind("a new note")
            .fetch_one(&pool)
            .await?;
        };

    //selecting all the notes
    pub fn select_rows() -> str{
        let rows =sqlx::query("SELECT * FROM notes").fetch_all(&pool).await?;
        // making the result string to pass to JS
        let str_result = rows
        .iter()
        .map(|r| format!("{} - {}",r.get::<i64,_>("id"),r.get::<String,_>("name")))
        .collect::<Vec<String>>()
        .join(", ");
        str_result
    };
    pub fn selection(){
        let select_query = sqlx::query("SELECT id, content FROM notes");
        let notes: Vec<Note> = select_query
        .map(|row: PgRow| Note{
            id: row.get("id"),
            content: row.get("content"),
        })
        .fetch_all(&pool)
        .await?;
    };
    Ok(())
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
