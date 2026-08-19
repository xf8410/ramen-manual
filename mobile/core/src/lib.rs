//! 手机版触屏决策层。
//!
//! 这一层不实现拉面杯规则，也不解析命令行输出。它负责把模拟器产生的
//! 当前待决策内容转换成 Android UI 可以展示的状态，并校验触屏提交。

use serde::{Deserialize, Serialize};

mod ramen_driver;
pub use ramen_driver::{action_options, decision_title, PortResult, RamenGameDriver, RamenGamePort};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DecisionOption {
    pub index: usize,
    pub title: String,
    #[serde(default)]
    pub detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DecisionKind {
    Ramen,
    SpecialFeeling,
    Training,
    Event,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PendingDecision {
    pub turn: u32,
    pub kind: DecisionKind,
    pub title: String,
    pub options: Vec<DecisionOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GameSummary {
    pub final_turn: u32,
    pub score: Option<i32>,
    pub pt: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MobileState {
    WaitingForStart,
    Decision(PendingDecision),
    Finished(GameSummary),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubmitError {
    NoDecisionPending,
    InvalidIndex { index: usize, option_count: usize },
}

pub trait RamenDriver {
    fn start(&mut self) -> Result<(), String>;
    fn advance_until_decision(&mut self) -> Result<PendingDecisionOrFinished, String>;
    fn submit(&mut self, index: usize) -> Result<PendingDecisionOrFinished, SubmitError>;
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PendingDecisionOrFinished {
    Decision(PendingDecision),
    Finished(GameSummary),
}

pub struct TouchSession<D> {
    driver: D,
    state: MobileState,
}

impl<D: RamenDriver> TouchSession<D> {
    pub fn new(driver: D) -> Self {
        Self { driver, state: MobileState::WaitingForStart }
    }

    pub fn state(&self) -> &MobileState { &self.state }

    pub fn start(&mut self) -> Result<(), String> {
        self.driver.start()?;
        self.advance()
    }

    pub fn submit(&mut self, index: usize) -> Result<(), SubmitError> {
        let option_count = match &self.state {
            MobileState::Decision(decision) => decision.options.len(),
            _ => return Err(SubmitError::NoDecisionPending),
        };
        if index >= option_count {
            return Err(SubmitError::InvalidIndex { index, option_count });
        }
        self.state = match self.driver.submit(index)? {
            PendingDecisionOrFinished::Decision(decision) => MobileState::Decision(decision),
            PendingDecisionOrFinished::Finished(summary) => MobileState::Finished(summary),
        };
        Ok(())
    }

    fn advance(&mut self) -> Result<(), String> {
        self.state = match self.driver.advance_until_decision()? {
            PendingDecisionOrFinished::Decision(decision) => MobileState::Decision(decision),
            PendingDecisionOrFinished::Finished(summary) => MobileState::Finished(summary),
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FakeDriver;

    impl RamenDriver for FakeDriver {
        fn start(&mut self) -> Result<(), String> { Ok(()) }
        fn advance_until_decision(&mut self) -> Result<PendingDecisionOrFinished, String> {
            Ok(PendingDecisionOrFinished::Decision(PendingDecision {
                turn: 1,
                kind: DecisionKind::Training,
                title: "选择行动".into(),
                options: vec![DecisionOption { index: 0, title: "速度训练".into(), detail: String::new() }],
            }))
        }
        fn submit(&mut self, _index: usize) -> Result<PendingDecisionOrFinished, SubmitError> {
            Ok(PendingDecisionOrFinished::Finished(GameSummary { final_turn: 1, score: None, pt: None }))
        }
    }

    #[test]
    fn touch_submission_is_checked_and_advances() {
        let mut session = TouchSession::new(FakeDriver);
        session.start().unwrap();
        assert!(matches!(session.state(), MobileState::Decision(_)));
        session.submit(0).unwrap();
        assert!(matches!(session.state(), MobileState::Finished(_)));
    }
}
