//! 真实上游 `RamenGame` 的非阻塞触屏适配器。
//!
//! 说明：动作阶段的 `RamenAction::apply` 只提交当前阶段选择，PC 入口随后调用
//! `Game::next()`；手机版也必须在提交动作后推进一次阶段，不能重新执行同一阶段。

use std::cell::RefCell;
use std::rc::Rc;

use anyhow::{anyhow, Result};
use rand::{rngs::StdRng, SeedableRng};
use umasim::game::ramen::{RamenAction, RamenGame, RamenStage};
use umasim::game::{ActionEnum, Game, Trainer};
use umasim::gamedata::{EventChoice, EventData};

use crate::{action_options, create_ramen_game, DecisionKind, GameSummary, PendingDecision, PortResult, RamenGamePort, RamenMobileConfig};

const PAUSED: &str = "mobile decision pending";
type SharedCapture = Rc<RefCell<Option<Capture>>>;
#[derive(Clone)]
enum Capture { Action { kind: DecisionKind, actions: Vec<RamenAction> }, Event { event: Option<EventData>, choices: Vec<Vec<EventChoice>> } }
struct PauseTrainer { capture: SharedCapture }
impl PauseTrainer { fn pause(&self, capture: Capture) -> Result<usize> { *self.capture.borrow_mut() = Some(capture); Err(anyhow!(PAUSED)) } }
impl Trainer<RamenGame> for PauseTrainer {
    fn select_action(&self, game: &RamenGame, actions: &[RamenAction], _rng: &mut StdRng) -> Result<usize> {
        let kind = match game.stage { RamenStage::RamenSelect => DecisionKind::Ramen, RamenStage::SpecialSelect => DecisionKind::SpecialFeeling, RamenStage::RegionSelect => DecisionKind::Region, RamenStage::SuperRamenSelect => DecisionKind::SuperRamen, _ => DecisionKind::Training };
        self.pause(Capture::Action { kind, actions: actions.to_vec() })
    }
    fn select_choice(&self, _game: &RamenGame, choices: &[Vec<EventChoice>], _rng: &mut StdRng) -> Result<usize> { self.pause(Capture::Event { event: None, choices: choices.to_vec() }) }
    fn select_event_choice(&self, _game: &RamenGame, event: &EventData, choices: &[Vec<EventChoice>], _rng: &mut StdRng) -> Result<usize> { self.pause(Capture::Event { event: Some(event.clone()), choices: choices.to_vec() }) }
}

pub struct RamenGameAdapter { game: Option<RamenGame>, rng: StdRng, capture: SharedCapture, config: RamenMobileConfig }
impl RamenGameAdapter {
    pub fn new(config: RamenMobileConfig) -> Self { Self { game: None, rng: StdRng::seed_from_u64(config.seed), capture: Rc::new(RefCell::new(None)), config } }
    pub fn config(&self) -> RamenMobileConfig { self.config }
    pub fn is_initialized(&self) -> bool { self.game.is_some() }
    fn game(&self) -> Result<&RamenGame, String> { self.game.as_ref().ok_or_else(|| "游戏尚未初始化".into()) }
    fn game_mut(&mut self) -> Result<&mut RamenGame, String> { self.game.as_mut().ok_or_else(|| "游戏尚未初始化".into()) }
    fn decision(capture: &Capture, turn: i32) -> PendingDecision {
        match capture {
            Capture::Action { kind, actions } => PendingDecision { turn: turn as u32, kind: kind.clone(), title: crate::decision_title(kind).into(), options: action_options(actions) },
            Capture::Event { event, choices } => PendingDecision { turn: turn as u32, kind: DecisionKind::Event, title: event.as_ref().map(|e| e.name.clone()).filter(|n| !n.is_empty()).unwrap_or_else(|| "选择事件选项".into()), options: choices.iter().enumerate().map(|(index, group)| crate::DecisionOption { index, title: format!("选项 {}", index + 1), detail: group.iter().map(|choice| choice.explain()).collect::<Vec<_>>().join(" | ") }).collect() },
        }
    }
    fn pump_inner(&mut self) -> Result<PortResult, String> {
        if self.game.is_none() { return Err("游戏尚未初始化".into()); }
        loop {
            let trainer = PauseTrainer { capture: self.capture.clone() };
            let result = { let game = self.game_mut()?; game.run_stage(&trainer, &mut self.rng) };
            if let Some(captured) = self.capture.borrow().as_ref() { let turn = self.game()?.turn(); return Ok(PortResult::Decision(Self::decision(captured, turn))); }
            if let Err(error) = result { return Err(error.to_string()); }
            if !self.game_mut()?.next() { return Ok(PortResult::Finished(self.summary())); }
        }
    }
    fn summary(&self) -> GameSummary { let game = self.game.as_ref().expect("summary requires game"); GameSummary { final_turn: game.turn() as u32, score: Some(game.uma.calc_score()), pt: Some(game.uma.total_pt()) } }
}
impl RamenGamePort for RamenGameAdapter {
    fn initialize(&mut self) -> Result<(), String> { let (game, rng) = create_ramen_game(self.config).map_err(|e| e.to_string())?; self.game = Some(game); self.rng = rng; self.capture.borrow_mut().take(); Ok(()) }
    fn pump(&mut self) -> Result<PortResult, String> { self.pump_inner() }
    fn choose(&mut self, index: usize) -> Result<PortResult, String> {
        let captured = self.capture.borrow_mut().take().ok_or_else(|| "当前没有待决策".to_string())?;
        match captured {
            Capture::Action { actions, .. } => {
                if index >= actions.len() { return Err(format!("动作索引越界: {} / {}", index, actions.len())); }
                let action = actions[index];
                let mut game = self.game.take().ok_or_else(|| "游戏尚未初始化".to_string())?;
                action.apply(&mut game, &mut self.rng).map_err(|e| e.to_string())?;
                self.game = Some(game);
                // PC run_full_game 在 run_stage 返回后调用 next()；动作提交也必须完成同样推进。
                self.game_mut()?.next();
            }
            Capture::Event { event, choices } => {
                if index >= choices.len() { return Err(format!("事件选项索引越界: {} / {}", index, choices.len())); }
                let event = event.ok_or_else(|| "事件数据缺失".to_string())?;
                let game = self.game_mut()?;
                game.apply_event(&event, index, &mut self.rng).map_err(|e| e.to_string())?;
            }
        }
        self.pump_inner()
    }
}
