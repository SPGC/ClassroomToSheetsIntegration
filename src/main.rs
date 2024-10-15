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
    // Чтение переменных окружения
    let task_name = env::var("INPUT_TASK_NAME")?;
    let student_github_id = env::var("INPUT_STUDENT_NAME")?;
    let robot_email = env::var("INPUT_ROBOT_EMAIL")?;
    let private_api_key = env::var("INPUT_PRIVATE_API_KEY")?;
    let task_result_str = env::var("INPUT_TASK_RESULTS")?;
    let table_id = env::var("INPUT_TABLE_ID")?;

    println!("{}", private_api_key);

    // Преобразование результата задания в число
    let task_result: i32 = task_result_str.parse()?;

    // Получение токена доступа
    let scope = "https://www.googleapis.com/auth/spreadsheets";
    let access_token = get_access_token(&robot_email, &private_api_key, scope).await?;

    // Имя листа (если нужно, можно тоже получить из переменной окружения)
    let sheet_name = "Sheet1";

    // Создание клиента
    let client = Client::new();

    // Создаем экземпляр StudentManager
    let student_manager = StudentManager::new(
        &client,
        &access_token,
        &table_id,
        sheet_name,
    );

    // Обновляем результат задания для студента
    student_manager
        .update_assignment_result(&student_github_id, &task_name, task_result)
        .await?;

    println!(
        "Результат задания '{}' для студента '{}' обновлен.",
        task_name, student_github_id
    );

    Ok(())
}
