fn test1() {
    let mut document = ini::Document::new();
    document.add("pets", "arthur", ini::Value::from_str("fox"));
    document.add("_", "spike", ini::Value::from_str("dog"));
    document.add("_", "katie", ini::Value::from_str("cat"));

    match document.write_to_disk("output.ini") {
        Ok(()) => println!("Document saved on output.ini"),
        Err(e) => println!("An error has occurred: {}", e),
    }
}

fn test2() {
    let document = ini::Document::build_from_file("sample.ini");

    match &document {
        Ok(d) => match d.get_value("pets", "arthur") {
            Some(v) => match v {
                ini::Value::String(v) => println!("{}", v),
                ini::Value::Int(v) => println!("{}", v),
                ini::Value::Float(v) => println!("{}", v),
            },
            None => println!("nothing"),
        }
        Err(e) => println!("{}", e),
    };
}

fn main() {
    test1();
}
