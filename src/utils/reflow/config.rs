use crate::core::config::FluffConfig;
use crate::utils::reflow::depth_map::DepthInfo;

#[derive(Debug, Clone)]
pub struct ReflowConfig {}

impl ReflowConfig {
    pub fn from_fluff_config(config: FluffConfig) -> ReflowConfig {
        panic!("Not implemented yet");
    }

    pub fn get_block_config (&self, block_class_types: Vec<String>, depth_info: Option<DepthInfo>) -> ReflowBlockConfig {
        panic!("Not implemented yet");
    }
}
