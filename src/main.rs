mod google_sheets;
mod data_processing;
mod students;
mod utils;

use google_sheets::auth::get_access_token;
use students::student_manager::StudentManager;
use reqwest::Client;
use tokio;
use std::env;
use utils::json_parser::{parse_results};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read environment variables
    let input_results_base64 = env::var("INPUT_RESULTS")?;
    let student_github_id = env::var("INPUT_STUDENT_NAME")?;
    let robot_email = env::var("INPUT_ROBOT_EMAIL")?;
    let private_api_key_raw = env::var("INPUT_PRIVATE_API_KEY")?;
    let table_id = env::var("INPUT_TABLE_ID")?;

    // Decode and parse the private key
    let private_api_key = private_api_key_raw.replace("\\n", "\n");

    // Decode and parse the test results
    let test_results = parse_results(&input_results_base64)?;

    // Get the access token
    let scope = "https://www.googleapis.com/auth/spreadsheets";
    let access_token = get_access_token(&robot_email, &private_api_key, scope).await?;

    // Sheet name
    let sheet_name = "Sheet1";

    // Client
    let client = Client::new();

    let student_manager = StudentManager::new(
        &client,
        &access_token,
        &table_id,
        sheet_name,
    );

    // Check the test results and update the student's grades
    for test in test_results.tests {
        let assignment_name = test.name;
        let result = if test.status == "pass" { 1 } else { 0 };

        student_manager
            .update_assignment_result(&student_github_id, &assignment_name, result)
            .await?;
    }

    Ok(())
}
