
#[derive(Serialize,Queryable, Debug)]
pub struct  Message {
    pub id: i32,
    pub username: String,
    pub message: String,
    pub timestamp: i64,
}