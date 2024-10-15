use crate::data_processing::utils::{
    find_first_empty_row, find_first_empty_column, find_column_by_header,
};
use crate::google_sheets::api::{read_from_sheet, write_to_cell};
use reqwest::Client;

pub struct StudentManager<'a> {
    pub client: &'a Client,
    pub access_token: &'a str,
    pub spreadsheet_id: &'a str,
    pub sheet_name: &'a str,
}

impl<'a> StudentManager<'a> {
    pub fn new(
        client: &'a Client,
        access_token: &'a str,
        spreadsheet_id: &'a str,
        sheet_name: &'a str,
    ) -> Self {
        StudentManager {
            client,
            access_token,
            spreadsheet_id,
            sheet_name,
        }
    }

    pub async fn get_or_create_student_row(
        &self,
        github_id: &str,
    ) -> Result<usize, Box<dyn std::error::Error>> {
        // Read the data from the table
        let read_range = format!("{}!A1:ZZ1000", self.sheet_name);
        let data = read_from_sheet(
            self.client,
            self.access_token,
            self.spreadsheet_id,
            &read_range,
        )
            .await?;

        // Parse the data into a table structure
        let table = crate::data_processing::parser::parse_sheet_data(&data)?;

        // Find the column index for 'github_id'
        let github_id_col = find_column_by_header(&table, "github_id")
            .ok_or("Столбец 'github_id' не найден")?;

        // Search for the student by GitHub ID
        for (row_idx, row) in table.iter().enumerate().skip(1) { // Пропускаем заголовок
            if let Some(cell_value) = row.get(github_id_col) {
                if cell_value == github_id {
                    // Студент найден, возвращаем индекс строки
                    return Ok(row_idx);
                }
            }
        }

        // Student not found, create a new record
        let new_row_idx = find_first_empty_row(&table);

        // Write the GitHub ID to the 'github_id' column in the new row
        write_to_cell(
            self.client,
            self.access_token,
            self.spreadsheet_id,
            new_row_idx,
            github_id_col,
            serde_json::json!(github_id),
        )
            .await?;

        Ok(new_row_idx)
    }

    pub async fn update_assignment_result(
        &self,
        github_id: &str,
        assignment_name: &str,
        result: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Find or create a student row
        let student_row = self.get_or_create_student_row(github_id).await?;

        // Read the data from the table (only the first row - headers)
        let header_range = format!("{}!A1:Z1", self.sheet_name);
        let data = read_from_sheet(
            self.client,
            self.access_token,
            self.spreadsheet_id,
            &header_range,
        )
            .await?;

        // Parse the data into a table structure
        let table = crate::data_processing::parser::parse_sheet_data(&data)?;

        let header_row = if !table.is_empty() {
            table[0].clone()
        } else {
            // If the table is empty, create a header with 'github_id'
            write_to_cell(
                self.client,
                self.access_token,
                self.spreadsheet_id,
                0,
                0,
                serde_json::json!("github_id"),
            )
                .await?;
            vec!["github_id".to_string()]
        };

        // Find the assignment column index
        let assignment_col = find_column_by_header(&table, assignment_name);

        let assignment_col = match assignment_col {
            Some(col_idx) => col_idx,
            None => {
                // Column not found, create a new column in the first empty column
                let new_col_idx = find_first_empty_column(&table);
                // Write the assignment name to the header
                write_to_cell(
                    self.client,
                    self.access_token,
                    self.spreadsheet_id,
                    0,
                    new_col_idx,
                    serde_json::json!(assignment_name),
                )
                    .await?;
                new_col_idx
            }
        };

        // Write the result to the cell at the intersection of the student row and the assignment column
        write_to_cell(
            self.client,
            self.access_token,
            self.spreadsheet_id,
            student_row,
            assignment_col,
            serde_json::json!(result),
        )
            .await?;

        Ok(())
    }
}
