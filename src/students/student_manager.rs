use crate::data_processing::utils::{
    find_first_empty_row, find_first_empty_column, find_column_by_header, coords_to_cell_address,
};
use crate::google_sheets::api::{read_from_sheet, write_to_cell};
use reqwest::Client;
use serde_json::Value;

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
        // Читаем данные из таблицы
        let read_range = format!("{}!A1:Z1000", self.sheet_name);
        let data = read_from_sheet(
            self.client,
            self.access_token,
            self.spreadsheet_id,
            &read_range,
        )
            .await?;

        // Преобразуем данные в структуру таблицы
        let table = crate::data_processing::parser::parse_sheet_data(&data)?;

        // Находим индекс столбца для 'github_id'
        let github_id_col = find_column_by_header(&table, "github_id")
            .ok_or("Столбец 'github_id' не найден")?;

        // Ищем строку с заданным GitHub ID
        for (row_idx, row) in table.iter().enumerate().skip(1) { // Пропускаем заголовок
            if let Some(cell_value) = row.get(github_id_col) {
                if cell_value == github_id {
                    // Студент найден, возвращаем индекс строки
                    return Ok(row_idx);
                }
            }
        }

        // Студент не найден, создаем новую запись
        let new_row_idx = find_first_empty_row(&table);

        // Записываем GitHub ID в столбец 'github_id' в новой строке
        write_to_cell(
            self.client,
            self.access_token,
            self.spreadsheet_id,
            new_row_idx,
            github_id_col,
            github_id,
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
        // Находим или создаем строку студента
        let student_row = self.get_or_create_student_row(github_id).await?;

        // Читаем данные из таблицы (только первую строку - заголовки)
        let header_range = format!("{}!A1:Z1", self.sheet_name);
        let data = read_from_sheet(
            self.client,
            self.access_token,
            self.spreadsheet_id,
            &header_range,
        )
            .await?;

        // Преобразуем данные в структуру таблицы
        let table = crate::data_processing::parser::parse_sheet_data(&data)?;

        let header_row = if !table.is_empty() {
            table[0].clone()
        } else {
            // Если таблица пуста, создаем заголовок с 'github_id'
            write_to_cell(
                self.client,
                self.access_token,
                self.spreadsheet_id,
                0,
                0,
                "github_id",
            )
                .await?;
            vec!["github_id".to_string()]
        };

        // Ищем индекс столбца задания
        let assignment_col = find_column_by_header(&table, assignment_name);

        let assignment_col = match assignment_col {
            Some(col_idx) => col_idx,
            None => {
                // Столбец не найден, создаем новый столбец в первом пустом столбце
                let new_col_idx = find_first_empty_column(&table);
                // Записываем название задания в заголовок
                write_to_cell(
                    self.client,
                    self.access_token,
                    self.spreadsheet_id,
                    0,
                    new_col_idx,
                    assignment_name,
                )
                    .await?;
                new_col_idx
            }
        };

        // Записываем результат в ячейку на пересечении строки студента и столбца задания
        write_to_cell(
            self.client,
            self.access_token,
            self.spreadsheet_id,
            student_row,
            assignment_col,
            &result.to_string(),
        )
            .await?;

        Ok(())
    }
}
