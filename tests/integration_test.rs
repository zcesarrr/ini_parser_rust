use ini::Value;

#[test]
fn document_manipulation() {
    let mut document = ini::Document::new();

    document.add("pets", "spike", ini::Value::from_str("dog"));
    document.add("pets", "arthur", ini::Value::from_str("fox"));
    document.add("pets", "katie", ini::Value::from_str("cat"));

    document.add("week", "monday", ini::Value::Int(1));
    document.add("week", "wednesday", ini::Value::Int(3));
    document.add("week", "friday", ini::Value::Int(5));

    let animal_type_of_spike = document.get_value("pets", "spike");
    if let None = animal_type_of_spike {
        panic!("spike from pets doesn't exist");
    }
    let animal_type_of_spike = animal_type_of_spike.unwrap();

    assert_eq!(document.section_exists("pets"), true);
    assert_eq!(document.section_exists("week"), true);
    assert_eq!(animal_type_of_spike, &ini::Value::String(String::from("dog")));
    assert_eq!(document.get_value("week", "friday").unwrap(), &ini::Value::Int(5));
}

#[test]
fn document_from_file() {
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("input.ini");
    let contents = format!("global=0\n[pets]\narthur=fox");

    if let Err(e) = std::fs::write(file_path.to_string_lossy().as_ref(), contents) {
        panic!("{}", e);
    }

    let document = ini::Document::build_from_file(&file_path.to_string_lossy());
    if let Err(e) = document {
        panic!("{}", e);
    }
    let mut document = document.unwrap();
    document.add("pets", "spike", Value::from_str("dog"));

    assert_eq!(document.section_exists("_"), true);
    assert_eq!(document.section_exists("pets"), true);
    assert_eq!(document.get_value("_", "global").unwrap(), &ini::Value::Int(0));
    assert_eq!(document.get_value("pets", "spike").unwrap(), &ini::Value::String(String::from("dog")));
    assert_eq!(document.get_value("pets", "arthur").unwrap(), &ini::Value::String(String::from("fox")));
}
