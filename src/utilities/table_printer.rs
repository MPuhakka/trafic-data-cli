use std::collections::HashMap;

pub struct TablePrinter {
    pub padding: usize,
}

impl TablePrinter {
    pub fn print(&self, data: &Vec<Vec<impl ToString>>) {
        let mut row_count: usize = 0;
        let mut col_size: HashMap<usize, usize> = HashMap::new();
        if data.len() == 0 {
            panic!("cannot print empty dataset");
        }
        for i in 0..data.len() {
            if row_count == 0 {
                row_count = data[i].len();
            } else if row_count != data[i].len() {
                panic!("row length anomaly at {i}");
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
                    width = col_size.get(&j).unwrap() + self.padding,
                );
            }
            print!("\n");
        }
    }
}
