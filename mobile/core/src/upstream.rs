//! PC 版 `ramen_manual.rs` 的手机版初始化迁移。

use anyhow::{Context, Result};
use rand::{rngs::StdRng, SeedableRng};
use umasim::game::{InheritInfo, ramen::RamenGame};
use umasim::gamedata::init_global_with_config;
use umasim::utils::load_game_config;

#[derive(Debug, Clone)]
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
            inherit: InheritInfo { blue_count: [15, 3, 0, 0, 0], extra_count: [0, 30, 0, 0, 30, 30] },
            seed: 20240816,
        }
    }
}

/// 与 PC `ramen_manual.rs` 相同：加载 game_config，校验拉面剧本，
/// 初始化全局数据，再创建原始 `RamenGame`。
pub fn create_ramen_game(config: RamenMobileConfig) -> Result<(RamenGame, StdRng)> {
    let game_config = load_game_config().context("加载 game_config.toml 失败")?;
    if game_config.scenario != "ramen" {
        anyhow::bail!("手机版要求 scenario=ramen，当前为 {:?}", game_config.scenario);
    }
    init_global_with_config(&game_config).context("初始化拉面杯全局数据失败")?;
    let game = RamenGame::newgame(config.uma_id, &config.deck, config.inherit)
        .context("创建 RamenGame 失败")?;
    Ok((game, StdRng::seed_from_u64(config.seed)))
}
