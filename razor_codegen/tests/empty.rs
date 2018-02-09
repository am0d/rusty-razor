use std::fmt;

pub struct Empty;

impl Empty {
    pub fn new() -> Empty {
        Empty        
    }
}

impl fmt::Display for Empty {
    fn fmt(&self, _out: &mut fmt::Formatter) -> fmt::Result {
    Ok(())
    }
}
