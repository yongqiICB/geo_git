use crate::geo::{color::Color, shape::Shape};

pub struct Commit {
    pub rect_actions: Vec<Action>,
    pub line_actions: Vec<Action>,
}

impl Default for Commit {
    fn default() -> Self {
        Self::new()
    }
}

impl Commit {
    pub fn new() -> Self {
        Self {
            line_actions: vec![],
            rect_actions: vec![],
        }
    }

    pub fn add_action(&mut self, action: Action) {
        self.rect_actions.push(action);
    }
    pub fn build(line_actions: Vec<Action>, rect_actions: Vec<Action>) -> Self {
        Self { line_actions, rect_actions }
    }
}

pub struct Action {
    pub action: ActionKind,
    pub name: String,
    pub geo: Shape,
    pub desc: Option<String>,
    pub color: Option<Color>,
    pub gradient: Option<f32>,
}


pub enum ActionKind {
    Add,
    Modify,
    Delete,
}
