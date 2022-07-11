use std::collections::HashMap;

pub struct TablePrinter {
    pub padding: Option<usize>,
}

impl TablePrinter {
    pub fn get_padding(&self) -> usize {
        self.padding.unwrap_or(1)
    }

    pub fn print(&self, data: &Vec<Vec<impl ToString>>) {
        let mut row_count: usize = 0;
        let mut col_size: HashMap<usize, usize> = HashMap::new();
        if data.len() == 0 {
            return;
        }
        for i in 0..data.len() {
            if row_count == 0 {
                row_count = data[i].len();
            } else if row_count != data[i].len() {
                return;
            }
            for j in 0..data[i].len() {
                let current_max = col_size.get(&j).unwrap_or(&0);
                let current_row_length = &data[i][j].to_string().len();

                let updated = if current_max < current_row_length {
                    current_row_length
                } else {
                    current_max
                };

                col_size.insert(j, updated.to_owned());
            }
        }
        for i in 0..data.len() {
            for j in 0..data[i].len() {
                print!(
                    "{:width$}",
                    data[i][j].to_string(),
                    width = col_size.get(&j).unwrap() + self.get_padding(),
                );
            }
            print!("\n");
        }
    }
}
