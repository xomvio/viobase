# Viobase
Viobase is an easy to use and easy to understand nosql db for Rust which is stores data in RAM and can aslo save as text file & gz encoded file

## Examples

### Creating a Table with data inside
* id is a must for now
```rust
fn create_products_table() -> VioTable {

    let cols = vec![
        Col {name: "id".to_string(), ty: VioType::Int},
        Col {name: "name".to_string(), ty: VioType::String},
        Col {name: "price".to_string(), ty: VioType::Int},
        Col {name: "stock".to_string(), ty: VioType::Int},
        Col {name: "category".to_string(), ty: VioType::String}
    ];

    let mut products = VioTable::create("products",cols);
    products.add_rows(
        vec![
            vec![Value::Int(0), Value::String("apple".to_string()), Value::Int(10), Value::Int(100), Value::String("fruit".to_string())],
            vec![Value::Int(1), Value::String("banana".to_string()), Value::Int(20), Value::Int(200), Value::String("fruit".to_string())],
            vec![Value::Int(2), Value::String("orange".to_string()), Value::Int(30), Value::Int(300), Value::String("fruit".to_string())],
            vec![Value::Int(3), Value::String("pear".to_string()), Value::Int(40), Value::Int(400), Value::String("fruit".to_string())],
    ]);

    products
}
```

### Creating a new Database and adding a table
```rust
let db = Viobase::new("db_name");
db.add_table(create_products_table());
```

### Creating a table from zero then editing and using
```rust
let mut db: VioBase = VioBase::new("my_db");
let mut users_table: VioTable = VioTable::new();
users_table.name = "users".to_string();

users_table.add_col("id", VioType::Int);
users_table.add_col("username", VioType::String);
users_table.add_col("password", VioType::String);

users_table.add_row(vec![
    Value::Int(0), 
    Value::String("admin".to_string()), 
    Value::String("bestpassword123".to_string())]);

users_table.add_row(vec![
    Value::Int(1), 
    Value::String("John".to_string()), 
    Value::String("pass123".to_string())]);

db.add_table(users_table);

let attempt_username = "admin".to_string();
let attempt_password = "wrongpassword987".to_string();

let mut authenticated = false;

//getting row that contains the id, username and password
let vals;
match db.tables[0].get("username", Value::String(attempt_username.to_owned())) {
    Some(x) => vals = x, //expected
    None => panic!("No user with that username"),        
}

//checking if password is correct
if attempt_password == vals[2].to_string() {
    authenticated = true;
}

if !authenticated { //expectedly
    //add new user
    db.tables[0].add_row(vec![
        Value::Int(2), 
        Value::String(attempt_username.to_string()), 
        Value::String(attempt_password.to_string())]);
}

```
