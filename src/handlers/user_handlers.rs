use axum ::{Json,extract::State,extract::Path};
use crate::db::AppState;
use crate::models::user::{User,NewUser};
use crate::response::wrapper::ResponseWrapper;
use crate::error::api_error::ApiError;
use rusqlite::params;

pub async fn create_user(
    State(state):State<AppState>,
    Json(payload):Json<NewUser>,
)->Result<Json<ResponseWrapper<User>>,ApiError>{

    let conn=state.db.lock().unwrap();

    let mut stmt=conn.prepare("INSERT INTO users(name,age)VALUSE(?1,?2)").unwrap();

    stmt.execute(params![payload.name, payload.age]).unwrap();

    let id=conn.last_insert_rowid() as i32;

    let user=User{id,name:payload.name,age:payload.age};

    Ok(Json(ResponseWrapper::success(user)))
}






pub async fn list_users(
    State(state):State<AppState>,
) -> Result<Json<ResponseWrapper<Vec<User>>>,ApiError>{
    let conn=state.db.lock().unwrap();
    let mut stmt=conn.prepare("SELECT id,name FROM users").unwrap();
    let rows = stmt.query_map([], |row| {
        Ok(User{
            id: row.get(0)?,
            name: row.get(1)?,
            age: row.get(2)?,
        })
    }).unwrap();

    let  users:Vec<User>=rows.map(|r|r.unwrap()).collect();

    Ok(Json(ResponseWrapper::success(users)))
}



pub async fn delete_user(
    State(state):State<AppState>,
    Path(id):Path<i32>,
)->Result<Json<ResponseWrapper<(String)>>,ApiError> {
    let conn = state.db.lock().unwrap();
    let rows_affected = conn.execute("DELETE FROM users WHERE id=?1", [id])
        .map_err(|_| ApiError::internal("Delete failed"))?;

    if rows_affected == 0 {
        return Err(ApiError::not_found("User not found"));
    }

    Ok(Json(ResponseWrapper::success("User deleted successfully".to_string())))

}
pub async fn get_user_by_id(
    State(state):State<AppState>,
    Path(id):Path<i32>,
)->Result<Json<ResponseWrapper<User>>,ApiError>{
    let conn=state.db.lock().unwrap();
    let mut stmt=conn.prepare("SELECT id,name,age FROM users WHERE id = ?1").unwrap();
    let user=stmt.query_row([id], |row|{
        Ok(User{
            id: row.get(0)?,
            name: row.get(1)?,
            age: row.get(2)?,
        })
    }).map_err(|_|ApiError::not_found("User not found"))?;

    Ok(Json(ResponseWrapper::success(user)))
}


pub async fn update_user(
    State(state):State<AppState>,
    Path(id):Path<i32>,
    Json(payload):Json<NewUser>,
)->Result<Json<ResponseWrapper<User>>,ApiError>{
    let conn=state.db.lock().unwrap();
    let mut stmt=conn.prepare("UPDATE users SET name=?1,age=?2 WHERE id = ?3").unwrap();
    stmt.execute(params![&payload.name, &payload.age,&id]).unwrap();
    let user=User{id,name:payload.name,age:payload.age};
    Ok(Json(ResponseWrapper::success(user)))
}