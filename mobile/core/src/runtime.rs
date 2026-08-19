//! Android 宿主准备上游 gamedata 的边界。

use std::path::{Path, PathBuf};
const REQUIRED: &[&str] = &["gamedata/cardDB.json", "gamedata/constants.json", "gamedata/default_config.toml", "gamedata/events.json", "gamedata/scenario_ramen.json", "gamedata/text_data_dict.json", "gamedata/umaDB.json", "game_config.toml"];
pub fn verify_ramen_data_dir(root: impl AsRef<Path>) -> Result<PathBuf, String> { let root = root.as_ref(); for relative in REQUIRED { let path = root.join(relative); if !path.is_file() { return Err(format!("缺少拉面杯运行数据: {}", path.display())); } } Ok(root.to_path_buf()) }
pub fn required_ramen_files() -> &'static [&'static str] { REQUIRED }
#[cfg(test)]
mod tests { use super::*; #[test] fn required_list_excludes_onsen() { assert!(!required_ramen_files().iter().any(|p| p.contains("onsen"))); } }
