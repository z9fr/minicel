use std::{env, process::exit};

use minicel::{read_file, usage, Table};

fn main() {
    let file_content = match env::args().nth(1) {
        Some(file_name) => read_file(&file_name),
        None => {
            usage();
            exit(1);
        }
    };

    Table::parse(&file_content, &env::args().nth(1).unwrap())
}
