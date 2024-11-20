use std::collections::BTreeMap;

use bytes::Bytes;

use super::version_controller::{Action, Commit, VersionId};
use crate::{
    geo::{self, color::Color, criticality::Criticality, shape::Shape},
    gui::public::Config,
};

#[derive(Clone, Debug)]
pub struct RectInfo {
    pub name: Bytes,
    pub geo: crate::geo::rect::Rect,
    pub color: Option<Color>,
    pub desc: Option<Bytes>,
}

#[derive(Clone, Debug)]
pub struct LineInfo {
    pub name: Bytes,
    pub geo: crate::geo::line::Line,
    pub color: Option<Color>,
    pub desc: Option<Bytes>,
}

impl VersionId {
    pub const GENESIS: Self = Self(0);
    pub fn incr(&self) -> Self {
        Self(self.0 + 1)
    }
}

#[derive(Debug)]
pub struct History<T: Clone>(pub BTreeMap<VersionId, Option<T>>);


impl<T: Clone> History<T> {
    pub fn new(create_time: VersionId, rect: T) -> Self {
        let mut res = BTreeMap::new();
        res.insert(create_time, Some(rect));
        Self(res)
    }

    pub fn update(&mut self, update_time: VersionId, rect: T) -> Option<T> {
        self.0.insert(update_time, Some(rect)).flatten()
    }

    pub fn del(&mut self, update_time: VersionId) {
        self.0.insert(update_time, None);
    }

    pub fn query(&self, time: VersionId) -> Option<T> {
        self.0.range(..=time).last().and_then(|(_, r)| r.clone())
    }
}

pub struct Db {
    pub version: VersionId,
    pub rects: BTreeMap<bytes::Bytes, History<RectInfo>>,
    pub lines: BTreeMap<bytes::Bytes, History<LineInfo>>,
    pub config: Config,
}

pub struct SlicedDb  {
    pub version: VersionId,
    pub rects: BTreeMap<Bytes, RectInfo>,
    pub lines: BTreeMap<bytes::Bytes, LineInfo>,
}


impl Db {
    pub fn new(cfg: Config) -> Self {
        Self {
            rects: BTreeMap::new(),
            lines: BTreeMap::new(),
            version: VersionId::GENESIS,
            config: cfg
        }
    }

    pub fn slice(&self, v: VersionId) -> SlicedDb {
        let rects = self
            .rects
            .iter()
            .filter_map(|(nid, hr)| hr.query(v).map(|x| (nid.clone(), x)))
            .collect();
        let lines = self
            .lines
            .iter()
            .filter_map(|(nid, hr)| hr.query(v).map(|x| (nid.clone(), x)))
            .collect();

        SlicedDb { version: v, rects, lines }
    }

    pub fn create_version(&mut self, commit: Commit) {
        self.version = self.version.incr();
        for v in commit.rect_actions {
            let action = v.action;
            let name = Bytes::copy_from_slice(v.name.as_bytes());
            match action {
                super::version_controller::ActionKind::Add => {
                    let color = match &self.config.color {
                        crate::geo::color::ColorType::RGB => v.color,
                        crate::geo::color::ColorType::Gradient {
                            generator,
                            min,
                            max,
                        } => v
                            .gradient
                            .map(|v| Criticality(v).color(*min, *max, generator)),
                    };

                    let desc = v.desc.map(|x| Bytes::copy_from_slice(x.as_bytes()));
                    match v.geo {
                        crate::geo::shape::Shape::Rect(rect) => {
                            assert!(self.rects.get_mut(&name).and_then(|history| history.query(self.version)).is_none());
                            let rect = RectInfo {
                                name: name.clone(),
                                geo: rect,
                                color,
                                desc,
                            };
                            self
                                .rects
                                .entry(name.clone())
                                .or_insert(History::new(self.version, rect.clone()))
                                .update(self.version, rect.clone());
                        },
                        crate::geo::shape::Shape::Line(line) => {
                            assert!(self.lines.get_mut(&name).and_then(|history| history.query(self.version)).is_none());
                            let line = LineInfo {
                                name: name.clone(),
                                geo: line,
                                color,
                                desc,
                            };
                            self
                                .lines
                                .entry(name.clone())
                                .or_insert(History::new(self.version, line.clone()))
                                .update(self.version, line.clone());
                        },
                        crate::geo::shape::Shape::None => {
                            panic!("To update a geometric must have its shape.")
                        },
                    }
                }
                super::version_controller::ActionKind::Modify => {
                    let histories = self.rects.get_mut(&name).unwrap();
                    let mut rect = histories.query(self.version).unwrap();
                    let mut diff = false;
                    let desc = v.desc.map(|x| Bytes::copy_from_slice(x.as_bytes()));
                    if desc != rect.desc {
                        diff = true;
                        rect.desc = desc;
                    }
                    if v.color != rect.color {
                        diff = true;
                        rect.color = v.color;
                    }
                    if let Shape::Rect(geo) = v.geo {    
                        if geo != rect.geo {
                            diff = true;
                            rect.geo = geo;
                        }
                    }
                    if diff {
                        histories.update(self.version, rect);
                    }
                }
                super::version_controller::ActionKind::Delete => {
                    assert!(self.rects.contains_key(&name));
                    let histories = self.rects.get_mut(&name).unwrap();
                    histories.del(self.version);
                }
            }
        }
        handle_line_action(self, commit.line_actions);
    }
}

pub fn handle_line_action(db: &mut Db, line_actions: Vec<Action>) {
    let mut handle_a_single_action = |action: Action| {
        let color = match &db.config.color {
            crate::geo::color::ColorType::RGB => action.color,
            crate::geo::color::ColorType::Gradient {
                generator,
                min,
                max,
            } => action
                .gradient
                .map(|v| Criticality(v).color(*min, *max, generator)),
        };

        let desc = action.desc.as_ref().map(|x| Bytes::copy_from_slice(x.as_bytes()));
        let name = Bytes::copy_from_slice(action.name.as_bytes());
        let geo::shape::Shape::Line(raw) = &action.geo else { panic!(); };
        let line = LineInfo {
            name: name.clone(),
            geo: raw.clone(),
            color,
            desc: desc.clone(),
        };
        match action.action {
            super::version_controller::ActionKind::Add => {
                db.lines.entry(name.clone()).or_insert(History::new(db.version, line.clone())).update(db.version, line);
            },
            super::version_controller::ActionKind::Modify => {
                let histories = db.lines.get_mut(&name).unwrap();
                let mut line_info = histories.query(db.version).unwrap();
                let mut diff = false;
                if desc != line_info.desc {
                    diff = true;
                    line_info.desc = desc;
                }
                if action.color != line_info.color {
                    diff = true;
                    line_info.color = action.color;
                }
                if let Shape::Line(geo) = action.geo {
                    if geo != line_info.geo {
                        diff = true;
                        line_info.geo = geo;
                    }
                }
                if diff {
                    histories.update(db.version, line_info);
                }
            },
            super::version_controller::ActionKind::Delete => {
                assert!(db.lines.contains_key(&name));
                let histories = db.lines.get_mut(&name).unwrap();
                histories.del(db.version);
            },
        }
    };
    for action in line_actions {
        handle_a_single_action(action);
    }
}