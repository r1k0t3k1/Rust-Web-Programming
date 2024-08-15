use std::vec::Vec;

use serde::Serialize;

use actix_web::{
    body::BoxBody, http::header::ContentType,
    HttpRequest, HttpResponse, Responder,
};

use crate::to_do::enums::TaskStatus;
use crate::to_do::{to_do_factory, ItemTypes};
use crate::to_do::structs::base::Base;
use crate::database::establish_connection;
use crate::diesel;
use diesel::prelude::*;
use crate::schema::to_do;
use crate::models::item::item::Item;

#[derive(Serialize)]
pub struct ToDoItems {
    pub pending_items: Vec<Base>,
    pub done_items: Vec<Base>,
    pub pending_item_count: i8,
    pub done_item_count: i8,
}

impl ToDoItems {
    pub fn new(input_items: Vec<ItemTypes>) -> Self {
        let mut pending_items = vec![];
        let mut done_items = vec![];

        for item in input_items {
            match item {
                ItemTypes::Done(packed) => done_items.push(packed.super_struct),
                ItemTypes::Pending(packed) => pending_items.push(packed.super_struct),
            }
        }

        let pending_item_count = pending_items.len() as i8;
        let done_item_count = done_items.len() as i8;

        ToDoItems { 
            pending_items,
            done_items,
            pending_item_count,
            done_item_count,
        }
    }

    pub fn get_state() -> ToDoItems {
        let mut conn = establish_connection();

        let results = to_do::table
            .order(to_do::columns::id.asc())
            .load::<Item>(&mut conn)
            .unwrap();
        
        let mut array_buffer = vec![]; 

        for item in results {
            let status = TaskStatus::from_string(item.status);
            let item = to_do_factory(&item.title, status);
            array_buffer.push(item);
        }
        
        ToDoItems::new(array_buffer)
    }
}

impl Responder for ToDoItems {
    type Body = BoxBody;

    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
       let body = serde_json::to_string(&self).unwrap();
       HttpResponse::Ok()
           .content_type(ContentType::json())
           .body(body)
    }
}
