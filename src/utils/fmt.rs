use super::{Value, VioType, VioTable};
use super::super::VioBase;

impl std::fmt::Display for VioType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            VioType::String => write!(f, "string"),
            VioType::Int => write!(f, "int"),
            VioType::Bool => write!(f, "bool"),
            VioType::Float => write!(f, "float"),
            VioType::Long => write!(f, "long"),
            VioType::Double => write!(f, "double"),
        }
    }    
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            //Value::String(ref v) => write!(f, "{}", from_utf8(v).unwrap()),
            Value::String(ref v) => write!(f, "{}", v),
            Value::Int(ref v) => write!(f, "{}", v),
            Value::Bool(ref v) => write!(f, "{}", v),
            Value::Float(ref v) => write!(f, "{}", v),
            Value::Long(ref v) => write!(f, "{}", v),
            Value::Double(ref v) => write!(f, "{}", v),
        }
    }
    
}

impl std::fmt::Display for VioTable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Table name: {}", &self.name)?;

        let mut colres = String::new();
        for i in 0..self.cols.len() {
            colres += format!("{} {},", &self.cols[i].name, &self.cols[i].ty).as_str();
        }
        colres.pop();
        writeln!(f, "{}", colres)?;

        for i in 0..self.rows.len() {
            let mut res: String = String::new();
            for value in &self.rows[i].values {
                match value {
                    Value::String(ref v) => {
                        res += v;
                        //res+=from_utf8(v).unwrap();
                    }
                    Value::Int(ref v) => {
                        res+=v.to_string().as_str();
                    }
                    Value::Bool(ref v) => {
                        res+=v.to_string().as_str();
                    }
                    Value::Float(ref v) => {
                        res+=v.to_string().as_str();
                    }
                    Value::Long(ref v) => {
                        res+=v.to_string().as_str();
                    }
                    Value::Double(ref v) => {
                        res+=v.to_string().as_str();
                    }
                }
                res += ",";
            }
            res.pop();
            writeln!(f, "{}", res)?;
        }
        writeln!(f, "end")?;

        Ok(())
    }
}

impl std::fmt::Display for VioBase {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Viobase name: {}", &self.name)?;
        for i in 0..self.tables.len() {
            writeln!(f, "{}", &self.tables[i])?;
        }
        Ok(())
    }
}