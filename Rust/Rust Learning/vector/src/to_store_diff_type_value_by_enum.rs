/* Generally vector can store only same types value,but if we want to store diff. type's val. then we can use "enum" */
#[derive(Debug)]

//In this "IntType() and many others" are not func. that is "enum variant with associated data", meaning of it just store value.
enum SpreadsheetCell {
    IntType(i32),
    FloatType(f64),
    CharType(char),
    StringType(String),
}

pub fn diff_types_val() {
    let v1 = vec![
        SpreadsheetCell::IntType(54),
        SpreadsheetCell::FloatType(15.32),
        SpreadsheetCell::CharType('J'),
        SpreadsheetCell::StringType(String::from("Jay")),
    ];

    let first_val = &v1[0];
    match first_val {
        SpreadsheetCell::IntType(val) => println!("Int:-{val}"),
        SpreadsheetCell::FloatType(val) => println!("Float:-{val}"),
        SpreadsheetCell::CharType(val) => println!("Char:-{val}"),
        SpreadsheetCell::StringType(val) => println!("Int:-{val}"),
    }
}
