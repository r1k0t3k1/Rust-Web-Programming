use std::borrow::{Borrow, BorrowMut};

use actix_web::{ web, HttpResponse };
use crate::json_serialization::{
    to_do_item::ToDoItem,
    to_do_items::ToDoItems,
};

use crate::diesel;
use diesel::prelude::*;
use crate::schema::to_do;
use crate::jwt::JwToken;
use crate::database::{DB, DB_CONNECTION};
use crate::database::establish_connection;

pub async fn edit(to_do_item: web::Json<ToDoItem>, token: JwToken, db: DB) -> HttpResponse {
    if &to_do_item.status != "DONE" && &to_do_item.status != "PENDING" {
        return HttpResponse::BadRequest().finish();
    }
   
    let mut conn = db.connection;
    let results = to_do::table
        .filter(to_do::columns::title.eq(&to_do_item.title));
     
    let _ = diesel::update(results)
        .set(to_do::columns::status.eq(&to_do_item.status))
        .execute(&mut conn);

    return HttpResponse::Ok().json(ToDoItems::get_state());
}
