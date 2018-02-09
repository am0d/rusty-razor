check: 
	cargo run -p razor_codegen -- -d tests
	cargo test --all

.PHONY: check clean run compiler
