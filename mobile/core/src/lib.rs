//! 手机版触屏决策层，规则来自上游 `RamenGame`。

use serde::{Deserialize, Serialize};

mod adapter;
mod protocol;
mod ramen_driver;
pub mod upstream;
pub use adapter::RamenGameAdapter;
pub use protocol::{SubmitMessage, UiDecision, UiMessage, UiState, UiStatus};
pub use ramen_driver::{action_options, decision_title, PortResult, RamenGameDriver, RamenGamePort};
pub use upstream::{create_ramen_game, RamenMobileConfig};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DecisionOption { pub index: usize, pub title: String, #[serde(default)] pub detail: String }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DecisionKind { Ramen, SpecialFeeling, Training, Region, SuperRamen, Event }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PendingDecision { pub turn: u32, pub kind: DecisionKind, pub title: String, pub options: Vec<DecisionOption> }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GameSummary { pub final_turn: u32, pub score: Option<i32>, pub pt: Option<i32> }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MobileState { WaitingForStart, Decision(PendingDecision), Finished(GameSummary) }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubmitError { NoDecisionPending, InvalidIndex { index: usize, option_count: usize } }
pub trait RamenDriver { fn start(&mut self) -> Result<(), String>; fn advance_until_decision(&mut self) -> Result<PendingDecisionOrFinished, String>; fn submit(&mut self, index: usize) -> Result<PendingDecisionOrFinished, SubmitError>; }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PendingDecisionOrFinished { Decision(PendingDecision), Finished(GameSummary) }
pub struct TouchSession<D> { driver: D, state: MobileState }
impl<D: RamenDriver> TouchSession<D> { pub fn new(driver: D) -> Self { Self { driver, state: MobileState::WaitingForStart } } pub fn state(&self) -> &MobileState { &self.state } pub fn ui_state(&self) -> UiState { (&self.state).into() } pub fn start(&mut self) -> Result<(), String> { self.driver.start()?; self.advance() } pub fn submit(&mut self, index: usize) -> Result<(), SubmitError> { let count = match &self.state { MobileState::Decision(d) => d.options.len(), _ => return Err(SubmitError::NoDecisionPending) }; if index >= count { return Err(SubmitError::InvalidIndex { index, option_count: count }); } self.state = match self.driver.submit(index)? { PendingDecisionOrFinished::Decision(d) => MobileState::Decision(d), PendingDecisionOrFinished::Finished(s) => MobileState::Finished(s) }; Ok(()) } fn advance(&mut self) -> Result<(), String> { self.state = match self.driver.advance_until_decision()? { PendingDecisionOrFinished::Decision(d) => MobileState::Decision(d), PendingDecisionOrFinished::Finished(s) => MobileState::Finished(s) }; Ok(()) } }

#[cfg(test)]
mod tests { use super::*; #[test] fn ui_state_exposes_pending_decision() { let d = PendingDecision { turn: 2, kind: DecisionKind::Ramen, title: "选择拉面".into(), options: vec![DecisionOption { index: 0, title: "不吃面".into(), detail: String::new() }] }; let state = MobileState::Decision(d); assert_eq!(UiState::from(&state).status, UiStatus::Decision); } }
