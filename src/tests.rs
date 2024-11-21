use poem::{
    EndpointExt, Route,
};
use std::sync::{Arc, Mutex};

#[cfg(test)]
mod tests {
    use super::*;
    use models::{AppState, MenuItem, Orders};
    use crate::{add_item, query_item, query_items, remove_item};
    use poem::test::TestClient;

    #[tokio::test]
    async fn test_add_item() {
        let state = AppState(Arc::new(Mutex::new(Orders::new())));
        let app = Route::new().at("/add", poem::post(add_item)).data(state.clone());

        let cli = TestClient::new(app);
        let menu_items = vec![
            MenuItem::new( "Burger".to_string(), 1, 10),
            MenuItem::new("Fries".to_string(), 1, 12)
        ];

        let resp = cli.post("/add").body_json(&menu_items).send().await;
        resp.assert_status_is_ok();
        let returned_items: Vec<MenuItem> = resp.json().await.value().deserialize();
        assert_eq!(returned_items, menu_items);
    }

    #[tokio::test]
    async fn test_remove_item() {
        let state = AppState(Arc::new(Mutex::new(Orders::new())));
        {
            let mut orders = state.0.lock().unwrap();
            orders.insert(
                1,
                vec![
                    MenuItem::new( "Burger".to_string(), 1, 10),
                    MenuItem::new("Fries".to_string(), 1, 12)
                ],
            );
        }
        let app = Route::new()
            .at("/remove/:table_number/:item_name", poem::delete(remove_item))
            .data(state.clone());

        let cli = TestClient::new(app);
        let resp = cli.delete("/remove/1/Burger").send().await;
        resp.assert_status_is_ok();
        let remaining_items: Vec<MenuItem> = resp.json().await.value().deserialize();
        assert_eq!(
            remaining_items,
            vec![MenuItem::new("Fries".to_string(), 1, 12)]
        );
    }

    #[tokio::test]
    async fn test_query_items() {
        let state = AppState(Arc::new(Mutex::new(Orders::new())));
        {
            let mut orders = state.0.lock().unwrap();
            orders.insert(
                1,
                vec![
                    MenuItem::new( "Burger".to_string(), 1, 10),
                    MenuItem::new("Fries".to_string(), 1, 12)
                ],
            );
        }
        let app = Route::new()
            .at("/query/:table_number", poem::get(query_items))
            .data(state.clone());

        let cli = TestClient::new(app);
        let resp = cli.get("/query/1").send().await;
        resp.assert_status_is_ok();
        let items: Vec<MenuItem> = resp.json().await.value().deserialize();
        assert_eq!(
            items,
            vec![
                MenuItem::new( "Burger".to_string(), 1, 10),
                MenuItem::new("Fries".to_string(), 1, 12)
            ]
        );
    }

    #[tokio::test]
    async fn test_query_item() {
        let state = AppState(Arc::new(Mutex::new(Orders::new())));
        {
            let mut orders = state.0.lock().unwrap();
            orders.insert(
                1,
                vec![MenuItem::new( "Burger".to_string(), 1, 10)],
            );
        }
        let app = Route::new()
            .at("/query/:table_number/:item_name", poem::get(query_item))
            .data(state.clone());

        let cli = TestClient::new(app);
        let resp = cli.get("/query/1/Burger").send().await;
        resp.assert_status_is_ok();
        let item: Option<MenuItem> = resp.json().await.value().deserialize();
        assert_eq!(
            item,
            Some(MenuItem::new( "Burger".to_string(), 1, 10))
        );
    }

}