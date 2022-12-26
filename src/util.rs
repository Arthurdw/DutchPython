pub mod file_util {
    use std::fs::File;
    use std::io::{Read, Write};

    pub fn read_file(path: &str) -> String {
        let mut file =
            File::open(path).expect("Kon het gevraagde bestand niet lezen, vraag hulp aan een coach!");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Kon de text in het bestand niet als text lezen, vraag hulp aan een coach!");
        contents
    }

    pub fn write_file(path: &str, contents: &str) {
        let mut file =
            File::create(path).expect("Kon geen nieuw bestand aanmaken, vraag hulp aan een coach!");
        file.write_all(contents.as_bytes())
            .expect("Kon geen text in het bestand schrijven, vraag hulp aan een coach!")
    }
}

pub mod json {
    use std::collections::HashMap;

    use serde_json::{from_str, Value};

    pub fn parse_json(json: &str) -> Value {
        from_str(json).expect("Kon de JSON niet parsen, vraag hulp aan een coach!")
    }

    pub fn into_hashmap(json: Value) -> HashMap<String, String> {
        let mut map = HashMap::new();

        if let Value::Object(obj) = json {
            for (key, value) in obj {
                if let Value::String(value) = value {
                    map.insert(key, value);
                }
            }
        }

        map
    }

    pub fn into_collection(json: HashMap<String, String>) -> Vec<(String, String)> {
        json.iter().map(|(key, value)| (key.to_string(), value.to_string())).collect()
    }
}
