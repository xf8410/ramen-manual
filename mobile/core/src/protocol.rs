//! Android 与 Rust 核心之间的稳定 JSON 协议。
//! UI 只消费快照和提交索引，不依赖 RamenAction 的 Rust 内部布局。

use serde::{Deserialize, Serialize};

use crate::{DecisionKind, DecisionOption, GameSummary, MobileState, PendingDecision};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum UiMessage {
    State { state: UiState },
    Error { message: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UiState {
    pub status: UiStatus,
    pub turn: Option<u32>,
    pub decision: Option<UiDecision>,
    pub summary: Option<GameSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum UiStatus { Waiting, Decision, Finished }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UiDecision {
    pub kind: DecisionKind,
    pub title: String,
    pub options: Vec<DecisionOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SubmitMessage {
    pub index: usize,
}

impl From<&MobileState> for UiState {
    fn from(state: &MobileState) -> Self {
        match state {
            MobileState::WaitingForStart => Self { status: UiStatus::Waiting, turn: None, decision: None, summary: None },
            MobileState::Decision(d) => Self {
                status: UiStatus::Decision,
                turn: Some(d.turn),
                decision: Some(UiDecision { kind: d.kind.clone(), title: d.title.clone(), options: d.options.clone() }),
                summary: None,
            },
            MobileState::Finished(summary) => Self { status: UiStatus::Finished, turn: Some(summary.final_turn), decision: None, summary: Some(summary.clone()) },
        }
    }
}

impl UiState {
    pub fn waiting() -> Self { MobileState::WaitingForStart.into() }
    pub fn from_decision(decision: &PendingDecision) -> Self { MobileState::Decision(decision.clone()).into() }
    pub fn finished(summary: &GameSummary) -> Self { MobileState::Finished(summary.clone()).into() }
}
