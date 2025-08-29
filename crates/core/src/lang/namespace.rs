use std::collections::HashMap;
use std::collections::hash_map::Iter;

/// Maps something like `common` to `Company.Common` in C# and similar.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NamespaceMappings {
    mappings: HashMap<String, String>,
}

impl NamespaceMappings {
    /// Creates a new mapping, assinging namespace id `""` to `default`.
    #[must_use]
    pub fn new(default: &str) -> Self {
        let mut mappings = HashMap::new();
        mappings.insert(String::new(), default.to_string());
        mappings.insert("_global".to_string(), default.to_string());

        Self { mappings }
    }

    /// Adds a mapping between namespace `id` to string `value`.
    #[must_use]
    pub fn add(mut self, id: &str, value: &str) -> Self {
        self.mappings.insert(id.to_string(), value.to_string());
        self
    }

    /// Returns the default namespace mapping
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn default_namespace(&self) -> &str {
        self.get("").expect("This must exist")
    }

    /// Obtains a mapping for the given ID.
    #[must_use]
    pub fn get(&self, id: &str) -> Option<&str> {
        self.mappings.get(id).map(String::as_str)
    }

    /// Iterates over all mappings.
    #[must_use]
    pub fn iter(&self) -> Iter<'_, String, String> {
        self.mappings.iter()
    }
}

impl<'a> IntoIterator for &'a NamespaceMappings {
    type Item = (&'a String, &'a String);
    type IntoIter = Iter<'a, String, String>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
