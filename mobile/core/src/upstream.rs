//! PC 版 `ramen_manual.rs` 的手机版初始化迁移。
//!
//! 这里保留 PC 版的拉面杯配置和 `RamenGame::newgame` 调用，不复制规则。

use anyhow::{Context, Result};
use rand::{rngs::StdRng, SeedableRng};
use umasim::game::{InheritInfo, RamenGame};
use umasim::gamedata::init_global;

/// 与 PC `ramen_manual.rs` 相同的默认配置。
#[derive(Debug, Clone, Copy)]
pub struct RamenMobileConfig {
    pub uma_id: u32,
    pub deck: [u32; 6],
    pub inherit: InheritInfo,
    pub seed: u64,
}

impl Default for RamenMobileConfig {
    fn default() -> Self {
        Self {
            uma_id: 102601,
            deck: [302424, 302894, 303044, 302924, 303024, 303054],
            inherit: InheritInfo {
                blue_count: [15, 3, 0, 0, 0],
                extra_count: [0, 30, 0, 0, 30, 30],
            },
            seed: 20240816,
        }
    }
}

/// 初始化全局游戏数据并创建真实的上游 `RamenGame`。
///
/// 调用方必须先把工作目录设置到包含上游数据文件的目录；这与 PC 入口
/// `check_working_dir()` 的运行前提一致。Android 宿主应在启动时准备 assets。
pub fn create_ramen_game(config: RamenMobileConfig) -> Result<(RamenGame, StdRng)> {
    init_global().context("初始化拉面杯全局数据失败")?;
    let game = RamenGame::newgame(config.uma_id, &config.deck, config.inherit)
        .context("创建 RamenGame 失败")?;
    Ok((game, StdRng::seed_from_u64(config.seed)))
}
