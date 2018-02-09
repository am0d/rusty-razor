UTIL_SRC = tests/*.rs.html

UTIL_OBJS = $(patsubst tests/%.rs.html,tests/%.rs,$(UTIL_SRC))

compiler: 
	cargo build

check: target/test
	diff test/index.expected.html test/index.actual.html

tests/%.rs: tests/%.rs.html
    $(CXX) $(CXXFLAGS) -o $@ $^
target/test: compiler test/test.rs 
	./target/debug/razor test/index.rs.html
	@rustc test/test.rs $(LINK_FLAGS) $(RUST_FLAGS) --test --out-dir target/test/

clean:
	cargo clean


.PHONY: check clean run compiler
