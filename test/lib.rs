use std::io;
pub trait Action {
    fn render(&self, out: &mut std::io::Write) -> std::io::Result<()>;
}
