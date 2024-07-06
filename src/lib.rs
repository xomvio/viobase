pub mod utils;
use utils::{Col, Value, VioTable, VioType};


fn main() {
    let cols:Vec<Col> = vec![
        Col { name: "name".to_string(), ty: VioType::String },
        Col { name: "age".to_string(), ty: VioType::Int },
        Col { name: "address".to_string(), ty: VioType::String },
    ];

    let mut person_table = VioTable::create("persons", cols);

    person_table.add_rows(
        vec![
            vec![
                Value::String("John".to_string()), 
                Value::Int(25), 
                Value::String("123 Main St".to_string())],
            vec![
                Value::String("Jane".to_string()), 
                Value::Int(31), 
                Value::String("123 Anne St".to_string())],
            vec![
                Value::String("Bob".to_string()), 
                Value::Int(25), 
                Value::String("123 Baba St".to_string())],
            vec![
                Value::String("Alice".to_string()), 
                Value::Int(42), 
                Value::String("123 Teyze St".to_string())],
        ]);

    person_table.add_row(vec![
        Value::String("Teoman".to_string()), 
        Value::Int(24), 
        Value::String("Istanbul Silivri".to_string())]);

    println!("{}", person_table);
    match person_table.get("name", Value::String("Bob".to_string())) {
        Some(v) => println!("{}", v[0]),
        None => println!("Not found"),
        
    }
}