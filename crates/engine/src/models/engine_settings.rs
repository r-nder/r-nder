pub struct EngineSettings {
    pub age: f64,
}

impl Default for EngineSettings {
    fn default() -> Self {
        Self {
            age: 0.0
        }
    }
}