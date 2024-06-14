use crate::geo::color::Color;

pub struct Commit {
    pub rect_actions: Vec<Action>,
}

impl Commit {
    pub fn new() -> Self {
        Self {
            rect_actions: vec![],
        }
    }

    pub fn add_action(&mut self, action: Action) {
        self.rect_actions.push(action);
    }
    pub fn build(rect_actions: Vec<Action>) -> Self {
        Self { rect_actions }
    }
}

pub struct Action {
    pub action: ActionKind,
    pub name: String,
    pub geo: Option<crate::geo::rect::Rect>,
    pub desc: Option<String>,
    pub color: Option<Color>,
}

impl Action {
    pub fn build(
        action: ActionKind,
        name: String,
        geo: Option<crate::geo::rect::Rect>,
        desc: Option<String>,
        color: Option<Color>,
    ) -> Self {
        match action {
            ActionKind::Add => {
                assert!(geo.is_some());
            }
            ActionKind::Modify => {}
            ActionKind::Delete => {
                assert!(geo.is_none());
                assert!(desc.is_none());
                assert!(color.is_none());
            }
        }

        Self {
            action,
            name,
            geo,
            desc,
            color,
        }
    }
}

pub enum ActionKind {
    Add,
    Modify,
    Delete,
}
