//! `RamenGame` 的手机版端口。

use crate::{DecisionKind, DecisionOption, GameSummary, PendingDecision, PendingDecisionOrFinished, RamenDriver, SubmitError};

pub trait RamenGamePort {
    fn initialize(&mut self) -> Result<(), String>;
    fn pump(&mut self) -> Result<PortResult, String>;
    fn choose(&mut self, index: usize) -> Result<PortResult, String>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PortResult { Decision(PendingDecision), Finished(GameSummary) }

pub struct RamenGameDriver<P> { port: P, started: bool }
impl<P> RamenGameDriver<P> {
    pub fn new(port: P) -> Self { Self { port, started: false } }
    pub fn port(&self) -> &P { &self.port }
    pub fn port_mut(&mut self) -> &mut P { &mut self.port }
    fn map_result(result: PortResult) -> PendingDecisionOrFinished { match result { PortResult::Decision(d) => PendingDecisionOrFinished::Decision(d), PortResult::Finished(s) => PendingDecisionOrFinished::Finished(s) } }
}
impl<P: RamenGamePort> RamenDriver for RamenGameDriver<P> {
    fn start(&mut self) -> Result<(), String> { self.port.initialize()?; self.started = true; Ok(()) }
    fn advance_until_decision(&mut self) -> Result<PendingDecisionOrFinished, String> { if !self.started { return Err("RamenGame 尚未初始化".into()); } self.port.pump().map(Self::map_result) }
    fn submit(&mut self, index: usize) -> Result<PendingDecisionOrFinished, SubmitError> {
        if !self.started { return Err(SubmitError::NoDecisionPending); }
        self.port.choose(index).map(Self::map_result).map_err(SubmitError::Backend)
    }
}

pub fn action_options<T: ToString>(actions: &[T]) -> Vec<DecisionOption> { actions.iter().enumerate().map(|(index, action)| DecisionOption { index, title: action.to_string(), detail: String::new() }).collect() }
pub fn decision_title(kind: &DecisionKind) -> &'static str { match kind { DecisionKind::Ramen => "选择拉面", DecisionKind::SpecialFeeling => "选择隐藏风味用法", DecisionKind::Training => "选择行动", DecisionKind::Region => "选择地区", DecisionKind::SuperRamen => "选择超级拉面", DecisionKind::Event => "选择事件" } }
