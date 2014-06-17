use std::io::IoResult;

pub trait Action {
    fn render(&self, out: &mut Writer) -> IoResult<()>;
}
