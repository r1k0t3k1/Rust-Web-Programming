use std::vec::Vec;

use serde::Serialize;
use serde_json::value::Value;
use serde_json::Map;

use actix_web::{
    body::BoxBody, http::header::ContentType,
    HttpRequest, HttpResponse, Responder,
};

use crate::to_do::ItemTypes;
use crate::to_do::structs::base::Base;
use crate::state::read_file;
use crate::to_do::{ to_do_factory, enums::TaskStatus };

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
        let state: Map<String, Value> = read_file("./state.json");
        let mut buf = vec![];

        for (k, v) in state {
            let status = TaskStatus::from_string(v.as_str().unwrap().to_string());
            let item = to_do_factory(&k, status);
            buf.push(item);
        }
        Self::new(buf)
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
