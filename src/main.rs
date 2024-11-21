mod tests;

use poem::{
    get,
    handler,
    listener::TcpListener,
    post,
    web::{Data, Json, Path},
    EndpointExt,
    Route,
    Server
};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{thread};
use models::{AppState, MenuItem, Orders};

#[handler]
async fn add_item(orders: Json<Vec<MenuItem>>, state: Data<&AppState>) -> Json<Vec<MenuItem>> {
    let mut state = state.0.0.lock().unwrap();
    let copy_orders = orders.clone();
    for item in copy_orders.0 {
        let table_orders = state.entry(item.table_number).or_insert_with(Vec::new);
        table_orders.push(item.clone());
    }
    Json(orders.0)
}

#[handler]
async fn remove_item(Path((table_number, item_name)): Path<(u32, String)>,  state: Data<&AppState>) -> Json<Vec<MenuItem>> {
    let mut state = state.0.0.lock().unwrap();
    if let Some(table_orders) = state.get_mut(&table_number) {
        table_orders.retain(|item| item.item_name != item_name);
    }
    Json(state.get(&table_number).cloned().unwrap_or_default())
}

#[handler]
async fn query_items(table_number: Path<u32>, state: Data<&AppState>) -> Json<Vec<MenuItem>> {
    let state = state.0.0.lock().unwrap();
    Json(state.get(&table_number.0).cloned().unwrap_or_default())
}

#[handler]
async fn query_item(Path((table_number, item_name)): Path<(u32, String)>,  state: Data<&AppState>) -> Json<Option<MenuItem>> {
    let state = state.0.0.lock().unwrap();
    if let Some(table_orders) = state.get(&table_number) {
        if let Some(item) = table_orders.iter().find(|item| item.item_name == item_name) {
            return Json(Some(item.clone()));
        }
    }
    Json(None)
}

async fn cleanup_expired_items(state: Arc<Mutex<Orders>>) {
    loop {
        thread::sleep(Duration::from_secs(5));
        let mut state = state.lock().unwrap();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        for table_orders in state.values_mut() {
            table_orders.retain(|item| now < item.created_at() + item.cooking_time);
        }
    }
}


#[tokio::main]
async fn main() {
    let state = AppState(Arc::new(Mutex::new(Orders::new())));

    let app = Route::new()
        .at("/add", post(add_item))
        .at("/remove/:table_number/:item_name", poem::delete(remove_item))
        .at("/query/:table_number", get(query_items))
        .at("/query/:table_number/:item_name", get(query_item))
        .data(state.clone());

    tokio::spawn(cleanup_expired_items(state.0.clone()));

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
        .unwrap();
}