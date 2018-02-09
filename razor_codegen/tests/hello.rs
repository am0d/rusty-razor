use std::fmt;

pub struct Hello;

impl Hello {
    pub fn new() -> Hello {
        Hello        
    }
}

impl fmt::Display for Hello {
    fn fmt(&self, _out: &mut fmt::Formatter) -> fmt::Result {
        _out.write_str(r###"<span>Hello from rusty-razor</span>"###)?;
    Ok(())
    }
}
