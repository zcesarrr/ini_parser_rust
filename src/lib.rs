#[cfg(test)]
mod tests;

use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub enum Value {
    String(String),
    Int(i32),
    Float(f32),
}

impl Value {
    pub fn new_string(text: &str) -> Self {
        Self::String(String::from(text))
    }

    pub fn from_str(str: &str) -> Self {
        if str.contains('.') {
            if let Ok(num) = str.parse() {
                return Self::Float(num);
            }
        }
        else {
            if let Ok(num) = str.parse() {
                return Self::Int(num);
            }
        }

        Self::new_string(str)
    }

    pub fn parse_str(&self) -> String {
        match self {
            Value::String(v) => v.to_string(),
            Value::Int(v) => v.to_string(),
            Value::Float(v) => v.to_string(),
        }
    }
}

pub struct Section {
    keys: HashMap<String, Value>,
}

impl Section {
    fn new() -> Self {
        Self {
            keys: HashMap::new(),
        }
    }

    fn add(&mut self, key: &str, value: Value) {
        let key: String = String::from(key);
        self.keys.insert(key, value);
    }

    fn get_value(&self, key: &str) -> Option<&Value> {
        self.keys.get(key)
    }
}

pub struct Document {
    sections: HashMap<String, Section>
}

impl Document {
    pub fn new() -> Self {
        Self {
            sections: HashMap::new(),
        }
    }

    pub fn build_from_str(contents: &str) -> Self {
        let mut document = Document::new();

        let mut section_name: &str = "_";
        'main: for line in contents.lines() {

            for (i, char) in line.chars().enumerate() {
                if i == 0 && char != '[' {
                    break;
                }

                if char == ']' && i == line.len() - 1 {
                    section_name = &line[1..i];
                    continue 'main;
                }
            }

            if line.contains('=') {
                let parts: Vec<&str> = line.split('=').collect();
                document.add(section_name, parts[0], Value::from_str(parts[1]));
            }
        }

        document
    }

    pub fn build_from_file(file_path: &str) -> Result<Self, std::io::Error> {
        let file_contents = std::fs::read_to_string(file_path)?;
        let document = Self::build_from_str(&file_contents);
        Ok(document)
    }

    pub fn add(&mut self, section: &str, key: &str, value: Value) {
        self.sections
            .entry(String::from(section))
            .or_insert(Section::new())
            .add(key, value);
    }

    pub fn get_value(&self, section: &str, key: &str) -> Option<&Value> {
        self.sections.get(section)?.get_value(key)
    }

    pub fn section_exists(&self, section: &str) -> bool {
        self.sections.get(section).is_some()
    }

    pub fn write_to_disk(&self, output_path: &str) -> Result<(), std::io::Error> {
        let mut contents: String = String::new();

        for (section_name, section) in self.sections.iter() {
            if section_name != "_" {
                contents.push_str(&format!("[{}]\n", section_name));
            }

            for (key, value) in section.keys.iter() {
                let key_value = &format!("{}={}\n", key, value.parse_str());
                
                if section_name == "_" {
                    contents.insert_str(0, key_value);
                } else {
                    contents.push_str(key_value);
                }
            }
        }

        std::fs::write(output_path, &contents)?;
        Ok(())
    }
}
