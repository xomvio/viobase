#[cfg(test)]
use super::{VioTable, VioType, Value};

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn test_table() {
    let mut table = VioTable::new();

    table.add_col("a", VioType::Int);
    table.add_col("b", VioType::String);
    table.add_col("c", VioType::Bool);
    table.add_col("d", VioType::Float);
    table.add_col("e", VioType::Long);
    table.add_col("f", VioType::Double);

    table.add_row(vec![Value::Int(1), Value::String("a".to_string()), Value::Bool(true), Value::Float(1.5), Value::Long(1), Value::Double(1.7)]);

    assert_eq!(table.name, "test".to_string());

    assert_eq!(table.rows.len(), 1);
    assert_eq!(table.rows[0].values.len(), 6);
    assert_eq!(table.rows[0].values[0].to_string(), "1".to_string());
    assert_eq!(table.rows[0].values[1].to_string(), "a".to_string());
    assert_eq!(table.rows[0].values[2].to_string(), "true".to_string());
    assert_eq!(table.rows[0].values[3].to_string(), "1.5".to_string());
    assert_eq!(table.rows[0].values[4].to_string(), "1".to_string());
    assert_eq!(table.rows[0].values[5].to_string(), "1.7".to_string());

    assert_eq!(table.cols.len(), 6);
    assert_eq!(table.cols[0].name, "a");
    assert_eq!(table.cols[0].ty.to_string(), "int");
    assert_eq!(table.cols[1].name, "b");
    assert_eq!(table.cols[1].ty.to_string(), "string");
    assert_eq!(table.cols[2].name, "c");
    assert_eq!(table.cols[2].ty.to_string(), "bool");
    assert_eq!(table.cols[3].name, "d");
    assert_eq!(table.cols[3].ty.to_string(), "float");
    assert_eq!(table.cols[4].name, "e");
    assert_eq!(table.cols[4].ty.to_string(), "long");
    assert_eq!(table.cols[5].name, "f");
    assert_eq!(table.cols[5].ty.to_string(), "double");

}
