mod client;

use rand::prelude::ThreadRng;
use crate::client::{add_item, remove_item, query_items, query_item};
use reqwest::Client;
use models::{MenuItem};
use tokio::task;
use rand::Rng;

#[tokio::main]
async fn main() {
    let client = Client::new();

    // Simulate 20 random requests
    let mut tasks = Vec::new();
    for _ in 0..20 {
        let task_client = client.clone();
        let mut rng: ThreadRng = rand::thread_rng();
        let num = rng.gen_range(0..5);
        let task = task::spawn(async move {
            match num {
                0 => {
                    // Add item
                    let items = vec![
                        MenuItem { item_name: format!("Pizza{}", num), table_number: 1, cooking_time: 10 },
                        MenuItem { item_name: format!("Burger{}", num), table_number: 2, cooking_time: 12 },
                    ];
                    let added_items = add_item(&task_client, items).await.unwrap();
                    println!("Added items: {:?}", added_items);
                }
                1 => {
                    // Query items
                    let queried_items = query_items(&task_client, 1).await.unwrap();
                    println!("Queried items for table 1: {:?}", queried_items);
                }
                2 => {
                    // Remove item
                    let remaining_items = remove_item(&task_client, 1, format!("Pizza{}", num)).await.unwrap();
                    println!("Remaining items for table 1: {:?}", remaining_items);
                }
                3 => {
                    // Query items
                    let queried_items = query_items(&task_client, 2).await.unwrap();
                    println!("Queried items for table 1: {:?}", queried_items);
                }
                4 => {
                    // Remove item
                    let remaining_items = remove_item(&task_client, 2, format!("Pizza{}", num)).await.unwrap();
                    println!("Remaining items for table 1: {:?}", remaining_items);
                }
                _ => unreachable!(),
            }
        });
        tasks.push(task);
    }

    // Wait for all tasks to complete
    for task in tasks {
        task.await.unwrap();
    }

    // Query items for table 1 after all operations
    let queried_items_after_removal = query_items(&client, 1).await.unwrap();
    println!("Queried items for table 1 after all operations: {:?}", queried_items_after_removal);
}
