RUST_FLAGS = -L . -O

LINK_FLAGS := -L build/

#TODO These two lines below should only need the first wildcard - work out why that isn't working ...
COMPILER_SOURCES := $(wildcard src/*.rs)
BINARIES := build/compiler

ALL_OBJS := $(ALL_SOURCES:src/%.rs=build/%.o)
ALL_TESTS := $(ALL_SOURCES:src/%.rs=build/%)

DEBUG_LIB := build/$(shell rustc src/my_debug.rs $(LINK_FLAGS) --out-dir=build --crate-type rlib --crate-file-name)

all: compiler

compiler: build/compiler

build/compiler: $(DEBUG_LIB) $(COMPILER_SOURCES)
	@echo Compiling $@
	@mkdir -p build/
	@rustc src/main.rs $(LINK_FLAGS) -o $@

$(DEBUG_LIB): src/my_debug.rs
	@echo Compiling $@
	@mkdir -p build/
	@rustc src/my_debug.rs $(LINK_FLAGS) --out-dir=build --crate-type rlib

run: $(BINARIES)
	./build/compiler test/index.rs.html

check: build/test
	@./$<
	diff test/index.expected.html test/index.actual.html

build/test: $(BINARIES) test/test.rs $(WEB_SOURCES)
	./build/compiler test/index.rs.html
	@echo Compiling $@ in test mode
	@rustc test/test.rs $(LINK_FLAGS) $(RUST_FLAGS) --test --out-dir build/

clean:
	@echo "Cleaning ..."
	@rm -f build/* $(BINARIES)

.PHONY: check clean run
