use std::io::IoResult;
pub mod index;

pub trait Action {
    fn render(&self, out: &mut Writer) -> IoResult<()>;
}
