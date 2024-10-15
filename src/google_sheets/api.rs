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
        println!("Error while loading data: {}", error_text);
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
        let _: Value = resp.json().await?;
        Ok(())
    } else {
        let error_text = resp.text().await?;
        println!("Writing error: {}", error_text);
        Err(Box::from(error_text))
    }
}


pub async fn write_to_cell(
    client: &Client,
    access_token: &str,
    spreadsheet_id: &str,
    row: usize,
    col: usize,
    value: Value, // Изменено с &str на Value
) -> Result<(), Box<dyn std::error::Error>> {
    // Преобразование координат в адрес ячейки
    let cell_address = crate::data_processing::utils::coords_to_cell_address(row, col);
    let range = cell_address.clone();

    // Проверяем и расширяем таблицу, если необходимо
    let sheet_id = 0; // Предположим, что работаем с первым листом
    let required_column_count = col + 1;
    let required_row_count = row + 1;

    // Получаем текущие размеры таблицы
    let (current_row_count, current_column_count) =
        crate::google_sheets::api::get_sheet_dimensions(client, access_token, spreadsheet_id, sheet_id).await?;

    // Расширяем количество столбцов, если необходимо
    if required_column_count > current_column_count {
        crate::google_sheets::api::expand_sheet_columns(
            client,
            access_token,
            spreadsheet_id,
            sheet_id,
            required_column_count as u32,
        )
            .await?;
    }

    // Расширяем количество строк, если необходимо
    if required_row_count > current_row_count {
        crate::google_sheets::api::expand_sheet_rows(
            client,
            access_token,
            spreadsheet_id,
            sheet_id,
            required_row_count as u32,
        )
            .await?;
    }

    // Формирование URL
    let url = format!(
        "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}?valueInputOption=USER_ENTERED",
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
        let _: Value = resp.json().await?;
        Ok(())
    } else {
        let error_text = resp.text().await?;
        println!("Error while writing data {}: {}", cell_address, error_text);
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
        let _: Value = resp.json().await?;
        Ok(())
    } else {
        let error_text = resp.text().await?;
        println!("Error while expanding the table: {}", error_text);
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
        Err("Can't find the sheet".into())
    } else {
        let error_text = resp.text().await?;
        println!("Error while loading table info {}", error_text);
        Err(Box::from(error_text))
    }
}

pub async fn get_sheet_dimensions(
    client: &Client,
    access_token: &str,
    spreadsheet_id: &str,
    sheet_id: u32,
) -> Result<(usize, usize), Box<dyn std::error::Error>> {
    // Формирование URL
    let url = format!(
        "https://sheets.googleapis.com/v4/spreadsheets/{}?fields=sheets.properties",
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
                    if properties.get("sheetId").and_then(|id| id.as_u64()) == Some(sheet_id as u64) {
                        let row_count = properties
                            .get("gridProperties")
                            .and_then(|gp| gp.get("rowCount"))
                            .and_then(|rc| rc.as_u64())
                            .unwrap_or(0) as usize;
                        let column_count = properties
                            .get("gridProperties")
                            .and_then(|gp| gp.get("columnCount"))
                            .and_then(|cc| cc.as_u64())
                            .unwrap_or(0) as usize;
                        return Ok((row_count, column_count));
                    }
                }
            }
        }
        Err("Can't get size of the sheet".into())
    } else {
        let error_text = resp.text().await?;
        println!("Error while loading size of the sheet {}", error_text);
        Err(Box::from(error_text))
    }
}


pub async fn expand_sheet_rows(
    client: &Client,
    access_token: &str,
    spreadsheet_id: &str,
    sheet_id: u32,
    new_row_count: u32,
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
                            "rowCount": new_row_count
                        }
                    },
                    "fields": "gridProperties.rowCount"
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
        let _: Value = resp.json().await?;
        Ok(())
    } else {
        let error_text = resp.text().await?;
        println!("Error while enlarging amount of rows: {}", error_text);
        Err(Box::from(error_text))
    }
}


