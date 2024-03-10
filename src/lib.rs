use std::fs::File;
use std::io::Read;

#[derive(Debug)]
enum ExprKind {
    CellKindText = 0,
    CellKindNumber,
    CellKindExpr,
    CellKindClone,
}

#[derive(Debug)]
pub struct Cell {
    pub file_row: usize,
    pub file_col: usize,
    pub string_view: String,
    pub exp_kind: ExprKind,
}

impl Cell {
    fn new(file_row: usize, file_col: usize, string_view: String, exp_kind: ExprKind) -> Self {
        Self {
            file_row,
            file_col,
            string_view,
            exp_kind,
        }
    }
}

#[derive(Debug)]
pub struct Table {
    pub cells: Vec<Cell>,
    pub rows: usize,
    pub cols: usize,
    pub file_path: String,
}

pub fn usage() {
    eprintln!("Usage: ./minicel <input.csv>")
}

pub fn read_file(file_name: &str) -> String {
    let mut data = String::new();
    let mut f = File::open(file_name).expect("Unable to open file");
    f.read_to_string(&mut data).expect("Unable to read string");
    return data;
}

impl Table {
    pub fn new(rows: usize, cols: usize, file_path: String) -> Self {
        Self {
            cells: Vec::new(),
            rows,
            cols,
            file_path,
        }
    }

    pub fn add_cell(&mut self, cell: Cell) {
        self.cells.push(cell);
    }

    pub fn parse(file_content: &str, file_name: &str) {
        let mut table = Table::new(0, 0, String::from(file_name));

        for (index, row) in file_content.split("\n").enumerate() {
            let row_index = index;
            for (col_index, col) in row.split("|").enumerate() {
                let trimmed_col = col.trim();

                let cell = Cell::new(row_index, col_index, trimmed_col.to_string());
                table.add_cell(cell);
            }
        }

        let max_row = table
            .cells
            .iter()
            .map(|cell| cell.file_row)
            .max()
            .unwrap_or(0);
        let max_col = table
            .cells
            .iter()
            .map(|cell| cell.file_col)
            .max()
            .unwrap_or(0);

        table.rows = max_row;
        table.cols = max_col;

        println!("{:?}", table)
    }
}

/*
* CSV parsing
*
* parse line by line and split each line by |
*
* typedef struct {
    Cell *cells;
    size_t rows;
    size_t cols;
    const char *file_path;
} Table;

0 A      | B
1 1      | 2
2 3      | 4
3 =A1+B1 | =A2+B2
*/
