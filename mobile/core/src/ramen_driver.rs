//! `RamenGame` 的手机版端口。
//!
//! 这里定义的是 PC 拉面杯程序与触屏层之间的替换点：PC 版的
//! `ManualTrainer + inquire` 不进入 Android，原 `RamenGame` 规则通过本端口被
//! 分段驱动。真正的上游类型绑定只放在 `RamenGameAdapter`，避免 UI 复制规则。

use crate::{DecisionKind, DecisionOption, GameSummary, PendingDecision, PendingDecisionOrFinished, RamenDriver, SubmitError};

/// 上游 RamenGame 适配器需要实现的最小操作。
///
/// `pump()` 必须只推进无需玩家输入的阶段；遇到 RamenSelect、SpecialSelect、
/// Train、RegionSelect、事件选项或 SuperRamenSelect 时返回 `Decision`。
pub trait RamenGamePort {
    fn initialize(&mut self) -> Result<(), String>;
    fn pump(&mut self) -> Result<PortResult, String>;
    fn choose(&mut self, index: usize) -> Result<PortResult, String>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PortResult {
    Decision(PendingDecision),
    Finished(GameSummary),
}

/// 将真实 `RamenGame` 适配器暴露给 TouchSession。
pub struct RamenGameDriver<P> {
    port: P,
    started: bool,
}

impl<P> RamenGameDriver<P> {
    pub fn new(port: P) -> Self {
        Self { port, started: false }
    }

    pub fn port(&self) -> &P {
        &self.port
    }

    pub fn port_mut(&mut self) -> &mut P {
        &mut self.port
    }

    fn map_result(result: PortResult) -> PendingDecisionOrFinished {
        match result {
            PortResult::Decision(decision) => PendingDecisionOrFinished::Decision(decision),
            PortResult::Finished(summary) => PendingDecisionOrFinished::Finished(summary),
        }
    }
}

impl<P: RamenGamePort> RamenDriver for RamenGameDriver<P> {
    fn start(&mut self) -> Result<(), String> {
        self.port.initialize()?;
        self.started = true;
        Ok(())
    }

    fn advance_until_decision(&mut self) -> Result<PendingDecisionOrFinished, String> {
        if !self.started {
            return Err("RamenGame 尚未初始化".into());
        }
        Ok(Self::map_result(self.port.pump()?))
    }

    fn submit(&mut self, index: usize) -> Result<PendingDecisionOrFinished, SubmitError> {
        if !self.started {
            return Err(SubmitError::NoDecisionPending);
        }
        self.port.choose(index).map(Self::map_result).map_err(|error| {
            // 端口错误不能伪装成非法索引；目前统一映射为无待决策，Android 层会显示错误。
            let _ = error;
            SubmitError::NoDecisionPending
        })
    }
}

/// 创建动作候选项的统一格式。名称来自 `RamenAction::Display`，不由 UI 重建。
pub fn action_options<T: ToString>(actions: &[T]) -> Vec<DecisionOption> {
    actions
        .iter()
        .enumerate()
        .map(|(index, action)| DecisionOption {
            index,
            title: action.to_string(),
            detail: String::new(),
        })
        .collect()
}

/// 为拉面杯阶段生成基本标题，便于真正的上游适配器保持一致。
pub fn decision_title(kind: &DecisionKind) -> &'static str {
    match kind {
        DecisionKind::Ramen => "选择拉面",
        DecisionKind::SpecialFeeling => "选择隐藏风味用法",
        DecisionKind::Training => "选择行动",
        DecisionKind::Event => "选择事件选项",
    }
}
