use std::collections::BTreeMap;

use bytes::Bytes;

use super::version_controller::{Commit, VersionId};
use crate::geo::color::Color;

#[derive(Clone, Debug)]
pub struct Rect {
    pub name: Bytes,
    pub geo: crate::geo::rect::Rect,
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
pub struct HistoryRect(pub BTreeMap<VersionId, Option<Rect>>);

impl HistoryRect {
    pub fn new(create_time: VersionId, rect: Rect) -> Self {
        let mut res = BTreeMap::new();
        res.insert(create_time, Some(rect));
        HistoryRect(res)
    }

    pub fn update(&mut self, update_time: VersionId, rect: Rect) -> Option<Rect> {
        self.0.insert(update_time, Some(rect)).flatten()
    }

    pub fn del(&mut self, update_time: VersionId) {
        self.0.insert(update_time, None);
    }

    pub fn query(&self, time: VersionId) -> Option<Rect> {
        self.0
            .range(..=time)
            .last()
            .and_then(|(_, r)| r.clone())
    }
}

pub struct Db {
    pub version: VersionId,
    pub rects: BTreeMap<bytes::Bytes, HistoryRect>,
}

pub struct SlicedDb {
    pub version: VersionId,
    pub rects: BTreeMap<Bytes, Rect>,
}
impl Default for Db {
    fn default() -> Self {
        Self::new()
    }
}

impl Db {
    pub fn new() -> Self {
        Self {
            rects: BTreeMap::new(),
            version: VersionId::GENESIS,
        }
    }

    pub fn slice(&self, v: VersionId) -> SlicedDb {
        let rects = self
            .rects
            .iter()
            .filter_map(|(nid, hr)| {
                hr.query(v).map(|x| (nid.clone(), x))
            })
            .collect();

        SlicedDb { version: v, rects }
    }

    pub fn create_version(&mut self, v: Commit) {
        self.version = self.version.incr();

        for v in v.rect_actions {
            let action = v.action;
            let name = Bytes::copy_from_slice(v.name.as_bytes());

            match action {
                super::version_controller::ActionKind::Add => {
                    assert!(self.rects.get_mut(&name).and_then(|history| history.query(self.version)).is_none());
                    let desc = v.desc.map(|x| Bytes::copy_from_slice(x.as_bytes()));
                    let geo = v.geo.unwrap();
                    let rect = Rect {
                        name: name.clone(),
                        geo,
                        color: v.color,
                        desc,
                    };
                    self.rects.entry(name.clone()).and_modify(|x| {x.update(self.version, rect.clone());}).or_insert(HistoryRect::new(self.version, rect));
                }
                super::version_controller::ActionKind::Modify => {
                    assert!(!self.rects.get_mut(&name).and_then(|history| history.query(self.version)).is_none());
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

                    if let Some(geo) = v.geo {
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
    }
}
