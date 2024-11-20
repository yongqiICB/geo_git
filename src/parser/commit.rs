use super::{r#impl::StringParser, tokens::next_ident};

pub fn next_commit(parser: &mut StringParser) {
    let res = next_ident(parser).unwrap();
    match res.to_ascii_uppercase().as_slice() {
        b"" => (),
        _ => unreachable!("What the hell have I just read, spit it out, sir."),
    }
}
