use std::collections::HashMap;

type SpecifierMap = HashMap<String, String>;

#[derive(Debug)]
pub struct PluginConfig {
    specifier_map: SpecifierMap,
}

impl PluginConfig {
    pub fn new(map: SpecifierMap) -> Self {
        if map.is_empty() {
            return Self::default();
        }

        Self { specifier_map: map }
    }

    pub fn get_specifier_map(&self) -> &SpecifierMap {
        &self.specifier_map
    }
}

impl Default for PluginConfig {
    fn default() -> Self {
        Self {
            specifier_map: HashMap::from([(String::from("ts"), String::from("js"))]),
        }
    }
}
