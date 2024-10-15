pub fn find_row_by_word(table: &Vec<Vec<String>>, word: &str) -> Option<usize> {
    for (i, row) in table.iter().enumerate() {
        if row.get(0).map_or(false, |cell| cell == word)
            || row.get(1).map_or(false, |cell| cell == word)
        {
            return Some(i);
        }
    }
    None
}

pub fn find_column_by_header(table: &Vec<Vec<String>>, header: &str) -> Option<usize> {
    if let Some(first_row) = table.get(0) {
        for (i, cell) in first_row.iter().enumerate() {
            if cell == header {
                return Some(i);
            }
        }
    }
    None
}

pub fn find_first_empty_row(table: &Vec<Vec<String>>) -> usize {
    for (i, row) in table.iter().enumerate() {
        if row.iter().all(|cell| cell.trim().is_empty()) {
            return i;
        }
    }
    table.len()
}

pub fn find_first_empty_column(table: &Vec<Vec<String>>) -> usize {
    if table.is_empty() {
        return 0;
    }

    let max_columns = table.iter().map(|row| row.len()).max().unwrap_or(0);

    for col_idx in 0..max_columns {
        let mut is_empty = true;
        for row in table {
            if let Some(cell) = row.get(col_idx) {
                if !cell.trim().is_empty() {
                    is_empty = false;
                    break;
                }
            }
        }
        if is_empty {
            return col_idx;
        }
    }
    max_columns
}

pub fn coords_to_cell_address(row: usize, col: usize) -> String {
    let column_letters = number_to_column_letters(col);
    let row_number = row + 1; // Таблица использует 1-индексацию для строк
    format!("{}{}", column_letters, row_number)
}

fn number_to_column_letters(mut col_num: usize) -> String {
    let mut letters = String::new();
    col_num += 1; // Таблица использует 1-индексацию для столбцов

    while col_num > 0 {
        let rem = (col_num - 1) % 26;
        letters.insert(0, (b'A' + rem as u8) as char);
        col_num = (col_num - 1) / 26;
    }
    letters
}

