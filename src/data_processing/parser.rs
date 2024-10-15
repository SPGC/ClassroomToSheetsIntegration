use serde_json::Value;

pub fn parse_sheet_data(data: &Value) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
    let mut table = Vec::new();
    if let Some(values) = data["values"].as_array() {
        for row in values {
            let mut row_data = Vec::new();
            if let Some(cells) = row.as_array() {
                for cell in cells {
                    row_data.push(cell.as_str().unwrap_or("").to_string());
                }
            }
            table.push(row_data);
        }
    }
    Ok(table)
}
