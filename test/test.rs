use std::io::Write;

pub mod index;

#[test]
fn test_index() {
    use std::fs::File;
    use std::path::Path;

    let mut file = match File::create(Path::new("test/index.actual.html")) {
        Ok(f) => f,
        Err(e) => panic!("Unable to create file `index.actual.html`: {}", e)
    };

    let model = vec! [(0, "First".to_string()), (1, "Second item".to_string())];
    let view = index::Index::new(model);
    // assert!(
    	write!(file, "{}", view);
    	 // == Ok(()));
}
