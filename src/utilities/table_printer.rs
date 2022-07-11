use std::{cell, collections::HashMap};

pub struct TablePrinter {
    pub padding: usize,
    pub header_underline: bool,
    pub column_separator: bool,
}

impl TablePrinter {
    pub fn print(&self, data: &Vec<Vec<impl ToString>>) {
        let mut row_count: usize = 0;
        let mut col_size: HashMap<usize, usize> = HashMap::new();
        let col_separator = if self.column_separator {
            Some('|')
        } else {
            None
        };
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
                let current_row_length = &data[i][j].to_string().chars().count();

                let updated = if current_max < current_row_length {
                    current_row_length
                } else {
                    current_max
                };

                col_size.insert(j, updated.to_owned());
            }
        }

        for i in 0..data.len() {
            // print underline
            // this works, I have no idea why it works
            // do not look
            if self.header_underline && i == 1 {
                let size = col_size.values().fold(
                    self.padding * (col_size.len() - 1)
                        + match col_separator {
                            Some(value) => col_size.len() * value.len_utf8() + self.padding,
                            None => 0,
                        },
                    |accumulator, current| accumulator + current,
                );
                for _ in 0..size {
                    print!("-");
                }
                print!("\n");
            }

            for j in 0..data[i].len() {
                let cell_width = col_size.get(&j).unwrap() + self.padding;
                let cell_value = data[i][j].to_string();
                match col_separator {
                    Some(it) => {
                        print!("{:width$}{}", cell_value, it, width = cell_width,);
                    }
                    None => {
                        print!("{:width$}", cell_value, width = cell_width,);
                    }
                }
            }
            print!("\n");
        }
    }
}
