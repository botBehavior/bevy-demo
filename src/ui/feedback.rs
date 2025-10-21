use bevy::prelude::*;
use std::time::Duration;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FeedbackKind {
    Success,
    Error,
    Info,
}

#[derive(Clone, Debug)]
pub struct FeedbackMessage {
    pub text: String,
    pub kind: FeedbackKind,
    timer: Timer,
}

impl FeedbackMessage {
    pub fn new(text: impl Into<String>, kind: FeedbackKind, seconds: f32) -> Self {
        let mut timer = Timer::from_seconds(seconds, TimerMode::Once);
        timer.unpause();
        Self {
            text: text.into(),
            kind,
            timer,
        }
    }

    pub fn tick(&mut self, delta: Duration) {
        self.timer.tick(delta);
    }

    pub fn finished(&self) -> bool {
        self.timer.finished()
    }
}

#[derive(Resource, Default, Debug)]
pub struct ShopFeedback {
    pub message: Option<FeedbackMessage>,
}

impl ShopFeedback {
    pub fn show(&mut self, text: impl Into<String>, kind: FeedbackKind, seconds: f32) {
        self.message = Some(FeedbackMessage::new(text, kind, seconds));
    }

    pub fn clear(&mut self) {
        self.message = None;
    }
}
