mod google_sheets;
mod data_processing;
mod students;

use google_sheets::auth::get_access_token;
use students::student_manager::StudentManager;
use reqwest::Client;
use serde_json::Value;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Загрузка ключа сервисного аккаунта из файла credentials.json
    let creds = std::fs::read_to_string("credentials.json")?;
    let creds_json: Value = serde_json::from_str(&creds)?;

    let private_key = creds_json["private_key"].as_str().unwrap();
    let client_email = creds_json["client_email"].as_str().unwrap();

    // Получение токена доступа
    let scope = "https://www.googleapis.com/auth/spreadsheets";
    let access_token = get_access_token(client_email, private_key, scope).await?;

    // Идентификатор таблицы и имя листа
    let spreadsheet_id = "129sP7Oi90QoE1dqhdA3KFPB1K54BJM5ot3WVPXOA43M";
    let sheet_name = "Sheet1"; // Имя листа

    // Создание клиента
    let client = Client::new();

    // Создаем экземпляр StudentManager
    let student_manager = StudentManager::new(
        &client,
        &access_token,
        spreadsheet_id,
        sheet_name,
    );

    // Пример использования функции update_assignment_result
    let github_id = "SPGC";
    let assignment_name = "task01";
    let result = 1;

    student_manager
        .update_assignment_result(github_id, assignment_name, result)
        .await?;

    let github_id = "spgc3";
    let assignment_name = "task02";
    let result = 4;

    student_manager
        .update_assignment_result(github_id, assignment_name, result)
        .await?;

    println!(
        "Результат задания '{}' для студента '{}' обновлен.",
        assignment_name, github_id
    );

    Ok(())
}
