use reqwest::Client;
use serde_json::Value;

pub async fn read_from_sheet(
    client: &Client,
    access_token: &str,
    spreadsheet_id: &str,
    range: &str,
) -> Result<Value, Box<dyn std::error::Error>> {
    // Формирование URL
    let url = format!(
        "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}",
        spreadsheet_id, range
    );

    // Отправка запроса
    let resp = client
        .get(&url)
        .bearer_auth(access_token)
        .send()
        .await?;

    if resp.status().is_success() {
        let data: Value = resp.json().await?;
        Ok(data)
    } else {
        let error_text = resp.text().await?;
        println!("Ошибка при получении данных: {}", error_text);
        Err(Box::from(error_text))
    }
}

pub async fn write_to_sheet(
    client: &Client,
    access_token: &str,
    spreadsheet_id: &str,
    range: &str,
    values: &Vec<Vec<String>>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Формирование URL
    let url = format!(
        "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}?valueInputOption=RAW",
        spreadsheet_id, range
    );

    // Тело запроса
    let body = serde_json::json!({
        "range": range,
        "majorDimension": "ROWS",
        "values": values
    });

    // Отправка запроса
    let resp = client
        .put(&url)
        .bearer_auth(access_token)
        .json(&body)
        .send()
        .await?;

    if resp.status().is_success() {
        let data: Value = resp.json().await?;
        println!("Данные успешно записаны: {:#?}", data);
        Ok(())
    } else {
        let error_text = resp.text().await?;
        println!("Ошибка при записи данных: {}", error_text);
        Err(Box::from(error_text))
    }
}

pub async fn write_to_cell(
    client: &Client,
    access_token: &str,
    spreadsheet_id: &str,
    row: usize,
    col: usize,
    value: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Преобразование координат в адрес ячейки
    let cell_address = crate::data_processing::utils::coords_to_cell_address(row, col);
    let range = cell_address.clone();

    // Формирование URL
    let url = format!(
        "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}?valueInputOption=RAW",
        spreadsheet_id, range
    );

    // Тело запроса
    let body = serde_json::json!({
        "range": range,
        "majorDimension": "ROWS",
        "values": [[value]]
    });

    // Отправка запроса
    let resp = client
        .put(&url)
        .bearer_auth(access_token)
        .json(&body)
        .send()
        .await?;

    if resp.status().is_success() {
        let data: Value = resp.json().await?;
        println!("Данные успешно записаны в ячейку {}: {:#?}", cell_address, data);
        Ok(())
    } else {
        let error_text = resp.text().await?;
        println!("Ошибка при записи данных в ячейку {}: {}", cell_address, error_text);
        Err(Box::from(error_text))
    }
}

pub async fn expand_sheet_columns(
    client: &Client,
    access_token: &str,
    spreadsheet_id: &str,
    sheet_id: u32,
    new_column_count: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    // Формирование URL
    let url = format!(
        "https://sheets.googleapis.com/v4/spreadsheets/{}:batchUpdate",
        spreadsheet_id
    );

    // Тело запроса
    let body = serde_json::json!({
        "requests": [
            {
                "updateSheetProperties": {
                    "properties": {
                        "sheetId": sheet_id,
                        "gridProperties": {
                            "columnCount": new_column_count
                        }
                    },
                    "fields": "gridProperties.columnCount"
                }
            }
        ]
    });

    // Отправка запроса
    let resp = client
        .post(&url)
        .bearer_auth(access_token)
        .json(&body)
        .send()
        .await?;

    if resp.status().is_success() {
        let data: Value = resp.json().await?;
        println!("Таблица успешно расширена: {:#?}", data);
        Ok(())
    } else {
        let error_text = resp.text().await?;
        println!("Ошибка при расширении таблицы: {}", error_text);
        Err(Box::from(error_text))
    }
}

pub async fn get_sheet_id_by_name(
    client: &Client,
    access_token: &str,
    spreadsheet_id: &str,
    sheet_name: &str,
) -> Result<u32, Box<dyn std::error::Error>> {
    // Формирование URL
    let url = format!(
        "https://sheets.googleapis.com/v4/spreadsheets/{}",
        spreadsheet_id
    );

    // Отправка запроса
    let resp = client
        .get(&url)
        .bearer_auth(access_token)
        .send()
        .await?;

    if resp.status().is_success() {
        let data: Value = resp.json().await?;
        if let Some(sheets) = data["sheets"].as_array() {
            for sheet in sheets {
                if let Some(properties) = sheet["properties"].as_object() {
                    if properties.get("title").and_then(|t| t.as_str()) == Some(sheet_name) {
                        if let Some(sheet_id) = properties.get("sheetId").and_then(|id| id.as_u64())
                        {
                            return Ok(sheet_id as u32);
                        }
                    }
                }
            }
        }
        Err("Не удалось найти лист с указанным именем".into())
    } else {
        let error_text = resp.text().await?;
        println!("Ошибка при получении информации о таблице: {}", error_text);
        Err(Box::from(error_text))
    }
}
