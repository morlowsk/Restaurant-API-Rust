use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use poem::{async_trait, Error, FromRequest, Request, RequestBody};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct MenuItem {
    pub item_name: String,
    pub table_number: u32,
    pub cooking_time: u64, // in seconds
    #[serde(skip_deserializing)]
    created_at: u64
}

impl Default for MenuItem {
    fn default() -> Self {
        Self {
            item_name: String::new(),
            table_number: 0,
            cooking_time: rand::thread_rng().gen_range(5..=15), // Default cooking time in seconds
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
        }
    }
}

impl MenuItem {
    pub fn new(item_name: String, table_number: u32, cooking_time: u64) -> Self {
        let created_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        Self {
            item_name,
            table_number,
            cooking_time,
            created_at
        }
    }

    pub fn new_with_default_time(item_name: String, table_number: u32) -> Self {
        Self {
            item_name,
            table_number,
            ..Default::default()
        }
    }

    // Getter method to access the private `created_at` field
    pub fn created_at(&self) -> u64 {
        self.created_at
    }
}


impl<'de> Deserialize<'de> for MenuItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct TempMenuItem {
            item_name: String,
            table_number: u32,
            #[serde(default = "default_cooking_time")]
            cooking_time: u64,
        }

        fn default_cooking_time() -> u64 {
            rand::thread_rng().gen_range(5..=15)
        }

        let temp = TempMenuItem::deserialize(deserializer)?;
        let created_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        Ok(MenuItem {
            item_name: temp.item_name,
            table_number: temp.table_number,
            cooking_time: temp.cooking_time,
            created_at,
        })
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