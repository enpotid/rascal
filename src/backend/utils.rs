use std::io::{Error, ErrorKind};


pub fn is_valid_name(scalname: String) -> std::io::Result<String> {
    let find = scalname.find(|c: char| {
        c.eq(&'\\')
            || c.eq(&'/')
            || c.eq(&':')
            || c.eq(&'*')
            || c.eq(&'?')
            || c.eq(&'"')
            || c.eq(&'|')
            || c.eq(&'<')
            || c.eq(&'>')
            || c.eq(&'\n')
            || c.eq(&'\r')
    });
    if find.is_some() {
        Err(Error::new(ErrorKind::InvalidInput, "Invalid name"))
    } else {
        Ok(scalname)
    }
}