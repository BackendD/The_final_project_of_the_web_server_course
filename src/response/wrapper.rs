use serde::Serialize;
#[derive(Serialize)]
pub struct ResponseWrapper<T> {
    pub success: bool,
    pub data: T,
}
impl<T> ResponseWrapper<T>{
    pub fn success(data: T)->Self{
        Self{success:true,data}
    }
}