pub struct SSEConfig {
    pub event: String,
    pub task: String,
}

pub fn sse_init_config() -> SSEConfig {
    SSEConfig {
        event: "event".to_string(),
        task: "task".to_string(),
    }
}
