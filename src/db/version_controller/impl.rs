use crate::geo::color::Color;

pub struct Commit {
    pub rect_actions: Vec<RectUpdater>,
}

impl Commit {
    pub fn build(rect_actions: Vec<RectUpdater>) -> Self {
        Self { rect_actions }
    }
}

pub struct RectUpdater {
    pub action: Action,
    pub name: String,
    pub geo: Option<crate::geo::rect::Rect>,
    pub desc: Option<String>,
    pub color: Option<Color>,
}

impl RectUpdater {
    pub fn build(
        action: Action,
        name: String,
        geo: Option<crate::geo::rect::Rect>,
        desc: Option<String>,
        color: Option<Color>,
    ) -> Self {
        match action {
            Action::Add => {
                assert!(geo.is_some());
            }
            Action::Modify => {}
            Action::Delete => {
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

pub enum Action {
    Add,
    Modify,
    Delete,
}
