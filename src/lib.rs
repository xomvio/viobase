pub mod utils;

use utils::{VioTable, VioType, Value};

use std::io::{Write, Read};

pub struct VioBase {
    pub name: String,
    pub tables: Vec<VioTable>,
}

impl VioBase {
    pub fn new(name: &str) -> VioBase {
        VioBase { name: name.to_string(), tables: vec![] }
    }

    pub fn create(name: &str, tables: Vec<VioTable>) -> VioBase {
        VioBase { name: name.to_string(), tables }
    }

    pub fn set_name(&mut self, name: &str) -> &VioBase {
        self.name = name.to_string();
        self
    }

    pub fn add_table(&mut self, table: VioTable) -> &VioBase {
        self.tables.push(table);
        self
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let mut file = std::fs::File::create("data.vio").unwrap();
        file.write_all(format!("{}", self).as_bytes()).unwrap();
        Ok(())
    }

    pub fn save_gz(&self) -> Result<(), std::io::Error> {
        let file = std::fs::File::create("data.gz").unwrap();
        let mut encoder = flate2::write::GzEncoder::new(file, flate2::Compression::default());
        encoder.write_all(format!("{}", self).as_bytes()).unwrap();
        Ok(())
    }

    pub fn load() -> VioBase {

        let mut file = std::fs::File::open("data.vio").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        VioBase::text_to_vb(contents)
    }

    pub fn load_gz() -> VioBase {
        let file = std::fs::File::open("data.gz").unwrap();
        let mut decoder = flate2::read::GzDecoder::new(file);
        let mut contents = String::new();
        decoder.read_to_string(&mut contents).unwrap();
        VioBase::text_to_vb(contents)
    }

    fn text_to_vb(contents: String) -> VioBase {
        let mut vb: VioBase = VioBase { name: String::new(), tables: vec![] };
        let mut last_table: VioTable = VioTable::new();

        for line in contents.lines() {
            if line.starts_with("Viobase name: ") {
                vb = VioBase { name: line[14..].to_string(), tables: vec![] };
            } 
            else if line.starts_with("Table name: ") {                
                last_table.name = line[12..].to_string();
            } 
            else if line.starts_with("id") {
                let cols_str = line.split(',').collect::<Vec<&str>>();
                for i in 0..cols_str.len() {
                    let col_str = cols_str[i].split(' ').collect::<Vec<&str>>();
                    let colname = col_str[0];
                    let colty = col_str[1];
                    match colty {
                        "bool" => last_table.add_col(colname, VioType::Bool),
                        "string" => last_table.add_col(colname, VioType::String),
                        "int" => last_table.add_col(colname, VioType::Int),
                        "float" => last_table.add_col(colname, VioType::Float),
                        "long" => last_table.add_col(colname, VioType::Long),
                        "double" => last_table.add_col(colname, VioType::Double),
                        _ => panic!("Unknown type: {}", colty),
                    };
                }
            }
            else if line == "end" {
                vb.add_table(last_table);
                last_table = VioTable::new();
            }
            else if line.is_empty() {

            }
            else {
                let row_str = line.split(',').collect::<Vec<&str>>();
                let mut row_values = vec![];
                for colid in 0..last_table.cols.len() {
                    match last_table.cols[colid].ty {
                        VioType::Bool => row_values.push(Value::Bool(row_str[colid].parse::<bool>().unwrap())),
                        VioType::String => row_values.push(Value::String(row_str[colid].to_string())),
                        VioType::Int => row_values.push(Value::Int(row_str[colid].parse::<i32>().unwrap())),
                        VioType::Float => row_values.push(Value::Float(row_str[colid].parse::<f32>().unwrap())),
                        VioType::Long => row_values.push(Value::Long(row_str[colid].parse::<i64>().unwrap())),
                        VioType::Double => row_values.push(Value::Double(row_str[colid].parse::<f64>().unwrap())),                        
                    }
                }
                last_table.add_row(row_values);
            }            
        }

        vb

    }

}