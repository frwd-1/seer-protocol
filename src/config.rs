// src/config.rs
pub struct Config {
    pub use_local_node: bool,
    pub alchemy_url: Option<String>,
    pub local_node_url: Option<String>,
}

impl Config {
    pub fn new(
        use_local_node: bool,
        alchemy_url: Option<String>,
        local_node_url: Option<String>,
    ) -> Self {
        Config {
            use_local_node,
            alchemy_url,
            local_node_url,
        }
    }
}
