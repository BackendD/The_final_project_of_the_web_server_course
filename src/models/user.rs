use serde::{Serialize,Deserialize};
#[derive(Serialize, Deserialize)]
pub struct User{
    pub id:i32,
    pub name:String,
    pub age:i32,
}

#[derive(Deserialize)]
pub struct NewUser{
    pub name:String,
    pub age:i32,
}