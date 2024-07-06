mod fmt;
pub mod tests;

pub enum VioType {
    Bool,
    String,
    Int,
    Float,
    Long,
    Double,
}

#[derive(PartialEq)]
pub enum Value {
    Bool(bool),
    String(String),
    Int(i32),
    Float(f32),
    Long(i64),
    Double(f64),
}

pub struct Row {
    pub index: usize,
    pub values: Vec<Value>,
}

pub struct Col {
    pub name: String,
    pub ty: VioType,
}


pub struct VioTable {
    pub name: String,
    pub cols: Vec<Col>,
    pub rows: Vec<Row>,
}

impl VioTable {
    pub fn new() -> VioTable {
        VioTable { name: String::new(), cols: vec![], rows: vec![] }
    }

    pub fn create(name: &str, cols: Vec<Col>) -> VioTable {
        VioTable { name: name.to_string(), cols, rows: vec![] }
    }

    pub fn add_col(&mut self, name: &str, ty: VioType) -> &VioTable {
        self.cols.push(Col { name: name.to_string(), ty });
        self
    }

    pub fn add_row(&mut self, values: Vec<Value>) -> &VioTable {
        self.rows.push(Row { index: self.rows.len(), values });
        self
    }

    pub fn add_rows(&mut self, rows: Vec<Vec<Value>>) -> &VioTable {
        for row in rows {
            self.add_row(row);
        }
        self
    }

    pub fn get(&self, val_name: &str, pair: Value) -> Option<&Vec<Value>> {
        if let Some(col_i) = self.cols.iter().position(|i| i.name == val_name) {
            for row in &self.rows {
                if row.values[col_i] == pair {
                    return Some(&row.values);
                }
            }
        }
        None
    }
}

impl Default for VioTable {
    fn default() -> Self {
        Self::new()
    }
}