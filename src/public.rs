use bytes::Bytes;

use crate::{db::{r#impl::Db, version_controller::VersionId}, lexer::Cursor, parser::r#impl::StringParser};

pub fn parse(x: String) -> Db {
    let mut db = Db::new();
    let cursor = Cursor::new(&x);
    let bytes = &Bytes::copy_from_slice(&x.as_bytes());
    let parser = StringParser::new(bytes, cursor);
    let commits = parser.parse();
    for c in commits {
        db.create_version(c);
    }
    db
}

#[test]
fn PP() {
    use std::{io::Read, path::PathBuf, str::FromStr};

    let prj_rt = project_root::get_project_root().unwrap();
    let mut file =
        std::fs::File::open(prj_rt.join(PathBuf::from_str("test/formal.txt").unwrap()))
            .unwrap();
    let mut raw_text = String::with_capacity(256);
    file.read_to_string(&mut raw_text).unwrap();

    let db = parse(raw_text);
    let v0 = db.slice(VersionId(0));
    let v1 = db.slice(VersionId(1));
    let v2 = db.slice(VersionId(2));
    let v3 = db.slice(VersionId(3));
    println!("[v0]");
    for (name, rect) in v0.rects.iter() {
        println!("{:?} - {:?}",name,rect);
    }

    println!("[v1]");
    for (name, rect) in v1.rects.iter() {
        println!("{:?} - {:?}",name,rect);
    }

    println!("[v2]");
    for (name, rect) in v2.rects.iter() {
        println!("{:?} - {:?}",name,rect);
    }

    println!("[v3]");
    for (name, rect) in v3.rects.iter() {
        println!("{:?} - {:?}",name,rect);
    }
}


#[test]
fn real_test() {
    use std::{io::Read, path::PathBuf, str::FromStr};

    let prj_rt = project_root::get_project_root().unwrap();
    let mut file =
        std::fs::File::open(prj_rt.join(PathBuf::from_str("test/formal.txt").unwrap()))
            .unwrap();
    let mut raw_text = String::with_capacity(256);
    file.read_to_string(&mut raw_text).unwrap();

    let db = parse(raw_text);
    let v0 = db.slice(VersionId(0));
    let v1 = db.slice(VersionId(1));
    let v2 = db.slice(VersionId(2));
    let v3 = db.slice(VersionId(3));
    println!("[v0]");
    for (name, rect) in v0.rects.iter() {
        println!("{:?} - {:?}",name,rect);
    }

    println!("[v1]");
    for (name, rect) in v1.rects.iter() {
        println!("{:?} - {:?}",name,rect);
    }

    println!("[v2]");
    for (name, rect) in v2.rects.iter() {
        println!("{:?} - {:?}",name,rect);
    }

    println!("[v3]");
    for (name, rect) in v3.rects.iter() {
        println!("{:?} - {:?}",name,rect);
    }
}