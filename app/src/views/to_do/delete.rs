use crate::diesel;
use crate::json_serialization::to_do_item::ToDoItem;
use crate::json_serialization::to_do_items::ToDoItems;
use diesel::prelude::*;
use actix_web::{web, HttpResponse};
use crate::database::establish_connection;
use crate::schema::to_do;
use crate::jwt::JwToken;
use crate::models::item::item::Item;

pub async fn delete(to_do_item: web::Json<ToDoItem>, token: JwToken) -> HttpResponse {
    let mut conn = establish_connection();

    let items = to_do::table
        .filter(to_do::columns::title.eq(&to_do_item.title))
        .order(to_do::columns::id.asc())
        .limit(1)
        .load::<Item>(&mut conn)
        .unwrap();

    let _ = diesel::delete(&items[0]).execute(&mut conn);

    return HttpResponse::Ok().json(ToDoItems::get_state());
}
