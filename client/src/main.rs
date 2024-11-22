mod client;
mod integration_tests;

use crate::client::{add_item, query_items, remove_item};
use models::MenuItem;
use rand::prelude::ThreadRng;
use rand::Rng;
use reqwest::Client;
use tokio::task;

#[tokio::main]
async fn main() {
    let client = Client::new();

    let food: Vec<String> = vec![
        String::from("Pizza"),
        String::from("Burger"),
        String::from("German Sausage"),
        String::from("Kung Pao Chicken"),
        String::from("Salad"),
        String::from("Ramen"),
        String::from("Taco")
    ];
    let food_len = food.len();

    // Simulate 20 random requests
    let mut tasks = Vec::new();
    for _ in 0..20 {
        let task_client = client.clone();
        let food = food.clone();
        let mut rng: ThreadRng = rand::thread_rng();
        let num = rng.gen_range(0..5);
        let food_idx = rng.gen_range(0..food_len);
        let task = task::spawn(async move {
            match num {
                0 => {
                    // Add item
                    let items = vec![
                        MenuItem::new_with_default_time(food[food_idx].clone(), 1),
                        MenuItem::new_with_default_time(food[(food_idx + 1) % 6].clone(), 2),
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
                    let remaining_items = remove_item(&task_client, 1, food[food_idx].clone()).await.unwrap();
                    println!("Remaining items for table 1: {:?}", remaining_items);
                }
                3 => {
                    // Query items
                    let queried_items = query_items(&task_client, 2).await.unwrap();
                    println!("Queried items for table 1: {:?}", queried_items);
                }
                4 => {
                    // Remove item
                    let remaining_items = remove_item(&task_client, 2, food[(food_idx + 1) % 6].clone()).await.unwrap();
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
