check: 
	cargo run -p razor_codegen -- -d razor_codegen/tests
	cargo test --all

.PHONY: check clean run compiler
