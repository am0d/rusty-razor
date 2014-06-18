#![feature(globs)]
pub use lib::Action;

pub mod lib;
pub mod index;

#[test]
fn test_index() {
    use std::io::File;

    let mut file = match File::create(&Path::new("test/index.actual.html")) {
        Ok(f) => f,
        Err(e) => fail!("Unable to create file `index.actual.html`: {}", e)
    };

    let model = vec! [(0, "First".to_string()), (1, "Second item".to_string())];
    let view = index::Index::new(model);
    assert!(view.render(&mut file as &mut Writer) == Ok(()));
}
