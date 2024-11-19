use models::MenuItem;
use reqwest::Client;
use std::error::Error;

pub async fn add_item(client: &Client, items: Vec<MenuItem>) -> Result<Vec<MenuItem>, Box<dyn Error>> {
    let response = client
        .post("http://localhost:3000/add")
        .json(&items)
        .send()
        .await?;

    let added_items: Vec<MenuItem> = response.json().await?;
    Ok(added_items)
}

pub async fn remove_item(client: &Client, table_number: u32, item_name: String) -> Result<Vec<MenuItem>, Box<dyn Error>> {
    let response = client
        .delete(&format!("http://localhost:3000/remove/{}/{}", table_number, item_name))
        .send()
        .await?;

    let remaining_items: Vec<MenuItem> = response.json().await?;
    Ok(remaining_items)
}

pub async fn query_items(client: &Client, table_number: u32) -> Result<Vec<MenuItem>, Box<dyn Error>> {
    let response = client
        .get(&format!("http://localhost:3000/query/{}", table_number))
        .send()
        .await?;

    let items: Vec<MenuItem> = response.json().await?;
    Ok(items)
}

pub async fn query_item(client: &Client, table_number: u32, item_name: String) -> Result<Option<MenuItem>, Box<dyn Error>> {
    let response = client
        .get(&format!("http://localhost:3000/query/{}/{}", table_number, item_name))
        .send()
        .await?;

    let item: Option<MenuItem> = response.json().await?;
    Ok(item)
}