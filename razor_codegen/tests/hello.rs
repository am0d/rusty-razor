use std::fmt;

pub struct Hello {
    model:  ()
}

impl Hello {
    pub fn new(m:  ()) -> Hello {
        Hello {
            model: m
        }
    }
}

impl fmt::Display for Hello {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        try!(out.write_str(r###"

<span>Hello from rusty-razor</span>"###));
    Ok(())
    }
}
