mod google_sheets;
mod data_processing;
mod students;

use google_sheets::auth::get_access_token;
use students::student_manager::StudentManager;
use reqwest::Client;
use serde_json::Value;
use tokio;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let task_name = env::var("INPUT_TASK_NAME")?;
    let student_github_id = env::var("INPUT_STUDENT_NAME")?;
    let robot_email = env::var("INPUT_ROBOT_EMAIL")?;
    let private_api_key_raw = env::var("INPUT_PRIVATE_API_KEY")?;
    let task_result_str = env::var("INPUT_TASK_RESULTS")?;
    let table_id = env::var("INPUT_TABLE_ID")?;

    let private_api_key = private_api_key_raw.replace("\\n", "\n");

    let task_result: i32 = task_result_str.parse()?;

    let scope = "https://www.googleapis.com/auth/spreadsheets";
    let access_token = get_access_token(&robot_email, &private_api_key, scope).await?;

    let sheet_name = env::var("INPUT_SHEET_NAME").unwrap_or("Sheet1".to_string());

    let client = Client::new();

    let student_manager = StudentManager::new(
        &client,
        &access_token,
        &table_id,
        &sheet_name,
    );

    student_manager
        .update_assignment_result(&student_github_id, &task_name, task_result)
        .await?;

    println!(
        "Результат задания '{}' для студента '{}' обновлен.",
        task_name, student_github_id
    );

    Ok(())
}
