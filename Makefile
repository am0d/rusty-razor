all: compiler

compiler: 
	cargo build -v

run: compiler 
	./target/razor test/index.rs.html

check: target/test
	./target/test/test
	diff test/index.expected.html test/index.actual.html

target/test: compiler test/test.rs 
	./target/razor test/index.rs.html
	@echo Compiling $@ in test mode
	@mkdir -p target/test
	@rustc test/test.rs $(LINK_FLAGS) $(RUST_FLAGS) --test --out-dir target/test/

clean:
	cargo clean


.PHONY: check clean run compiler
