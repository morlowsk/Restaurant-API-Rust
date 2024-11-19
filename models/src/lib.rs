use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use poem::{async_trait, Error, FromRequest, Request, RequestBody};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MenuItem {
    pub item_name: String,
    pub table_number: u32,
    pub cooking_time: u32 // in minutes
}

impl MenuItem {
    fn new(item_name: String, table_number: u32) -> Self {
        let cooking_time = rand::thread_rng().gen_range(5..=15);
        Self {
            item_name,
            table_number,
            cooking_time,
        }
    }
}

pub type Orders = HashMap<u32, Vec<MenuItem>>; // Table number -> List of MenuItems

#[derive(Clone)]
pub struct AppState(pub Arc<Mutex<Orders>>);

#[async_trait]
impl<'a> FromRequest<'a> for AppState {
    async fn from_request(_req: &'a Request, _body: &mut RequestBody) -> Result<Self, Error> {
        // In a real app, extract the shared state from the request extensions or app context.
        Ok(AppState(Arc::new(Mutex::new(HashMap::new()))))
    }
}
