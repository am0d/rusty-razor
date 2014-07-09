all: compiler

compiler: 
	cargo build

run: compiler 
	./target/compiler test/index.rs.html

check: target/test
	cargo test
	./target/test/test
	diff test/index.expected.html test/index.actual.html

target/test: compiler test/test.rs 
	./target/compiler test/index.rs.html
	@echo Compiling $@ in test mode
	@rustc test/test.rs $(LINK_FLAGS) $(RUST_FLAGS) --test --out-dir target/test/

clean:
	cargo clean


.PHONY: check clean run compiler
