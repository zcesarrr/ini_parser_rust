use super::*;

fn sample_string_contents() -> String {
    "[games_rating]\nsilksong=90%\noneshot=88\n[stores_availability]\nsteam=yes\nepicgames=no\ngamejolt=11.2".to_string()
}

fn sample_document() -> Document {
    let mut document = Document::new();
    document.add("games_rating", "silksong", Value::from_str("90%"));
    document.add("games_rating", "oneshot", Value::Int(88));
    document.add("stores_availability", "steam", Value::from_str("yes"));
    document.add("stores_availability", "epicgames", Value::from_str("yes"));
    document.add("stores_availability", "gamejolt", Value::Float(11.2));

    document
}

#[test]
fn parsing_contents_to_document() {
    let contents = sample_string_contents();
    let document = Document::build_from_str(&contents);
    
    assert_eq!(document.section_exists("games_rating"), true);
    assert_eq!(document.section_exists("stores_availability"), true);
    assert_eq!(document.section_exists("farming"), false);
    assert_eq!(document.get_value("games_rating", "silksong").unwrap().parse_str(), "90%");
    assert_eq!(document.get_value("games_rating", "oneshot").unwrap(), &Value::Int(88));
    assert_eq!(document.get_value("stores_availability", "steam").unwrap(), &Value::from_str("yes"));
    assert_eq!(document.get_value("stores_availability", "epicgames").unwrap(), &Value::String(String::from("no")));
    assert_eq!(document.get_value("stores_availability", "gamejolt").unwrap(), &Value::Float(11.2));
}

#[test]
fn getting_none_on_inexisting_value() {
    let document = sample_document();
    assert_eq!(document.get_value("games", "ori"), None);
}

#[test]
fn invalid_contents_parsing() {
    let contents = "hello\n[hi\nhey\n[section]key=value\nvalid=0";
    let document = Document::build_from_str(contents);

    assert_eq!(document.get_value("_", "hello"), None);
    assert_eq!(document.get_value("hi", "hey"), None);
    assert_eq!(document.get_value("_", "hey"), None);
    assert_eq!(document.section_exists("section"), false);
    assert_eq!(document.get_value("_", "valid"), Some(&Value::Int(0)));
}

#[test]
fn parsing_empty_value_to_string() {
    let contents = "empty=";
    let document = Document::build_from_str(contents);

    assert_eq!(document.get_value("_", "empty"), Some(&Value::from_str("")));
}

#[test]
fn parsing_file_contents() {
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("input.ini");
    let contents = format!("global=0\n{}", sample_string_contents());

    let res = std::fs::write(file_path.to_string_lossy().as_ref(), contents);
    assert_eq!(res.unwrap(), ());

    let document = Document::build_from_file(&file_path.to_string_lossy()).expect("Should document has been created");

    assert_eq!(document.section_exists("games_rating"), true);
    assert_eq!(document.get_value("games_rating", "silksong").unwrap(), &Value::from_str("90%"));
    assert_eq!(document.section_exists("_"), true);
    assert_eq!(document.get_value("_", "global").unwrap(), &Value::Int(0));
}

#[test]
fn writing_to_disk() {
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("output.ini");

    let document = sample_document();

    let result = document.write_to_disk(&file_path.to_string_lossy());
    assert_eq!(result.unwrap(), ());

    let loaded_document = Document::build_from_file(&file_path.to_string_lossy()).unwrap();
    assert_eq!(loaded_document.section_exists("games_rating"), true);
    assert_eq!(loaded_document.get_value("games_rating", "silksong").unwrap(), &Value::from_str("90%"));
}

#[test]
fn global_section_is_not_present_on_output_file() {
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("output.ini");

    let mut document = Document::new();
    document.add("_", "spike", Value::from_str("dog"));
    document.add("pets", "arthur", Value::from_str("fox"));

    let result = document.write_to_disk(&file_path.to_string_lossy());
    assert_eq!(result.unwrap(), ());

    let contents = std::fs::read_to_string(&file_path).unwrap();
    assert_eq!(contents.contains("[_]"), false);

    let loaded_document = Document::build_from_file(&file_path.to_string_lossy()).unwrap();
    assert_eq!(loaded_document.section_exists("_"), true);
    assert_eq!(loaded_document.section_exists("pets"), true);
}
