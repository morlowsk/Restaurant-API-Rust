#[cfg(test)]
mod tests {
    use std::time::Duration;
    use reqwest::Client;
    use models::MenuItem;

    #[tokio::test]
    async fn e2e_it_add_query_remove_test() {
        let client = Client::new();

        // Add item
        let items = vec![
            MenuItem::new("Pizza".to_string(), 1, 5),
        ];
        let added_items = client
            .post("http://localhost:3000/add")
            .json(&items)
            .send()
            .await
            .unwrap()
            .json::<Vec<MenuItem>>()
            .await
            .unwrap();
        assert_eq!(added_items.len(), 1);

        // Query items for table 1
        let queried_items = client
            .get("http://localhost:3000/query/1")
            .send()
            .await
            .unwrap()
            .json::<Vec<MenuItem>>()
            .await
            .unwrap();
        assert_eq!(queried_items.len(), 1);

        // Wait for the item to be removed
        tokio::time::sleep(Duration::from_secs(10)).await;

        // Query items for table 1 after removal
        let queried_items_after_removal = client
            .get("http://localhost:3000/query/1")
            .send()
            .await
            .unwrap()
            .json::<Vec<MenuItem>>()
            .await
            .unwrap();
        assert_eq!(queried_items_after_removal.len(), 0);
    }

}