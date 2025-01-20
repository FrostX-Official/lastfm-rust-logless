use std::collections::HashMap;

#[derive(Default)]
pub struct ParameterBuilder {
    params: HashMap<String, String>,
}

impl ParameterBuilder {
    pub fn new() -> Self {
        ParameterBuilder {
            params: HashMap::new(),
        }
    }

    pub fn add(mut self, key: &str, value: String) -> Self {
        self.params.insert(key.to_string(), value);
        self
    }

    pub fn add_optional<T: Into<String>>(mut self, key: &str, value: Option<T>) -> Self {
        if let Some(val) = value {
            self = self.add(key, val.into());
        }
        self
    }

    pub fn build(self) -> HashMap<String, String> {
        self.params
    }
}
