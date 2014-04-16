RUST_FLAGS = -L . -O

LINK_FLAGS := -L build/

#TODO These two lines below should only need the first wildcard - work out why that isn't working ...
COMPILER_SOURCES := $(wildcard src/*.rs)
BINARIES := build/compiler

ALL_OBJS := $(ALL_SOURCES:src/%.rs=build/%.o)
ALL_TESTS := $(ALL_SOURCES:src/%.rs=build/%)

all: compiler

compiler: build/compiler

build/compiler: $(COMPILER_SOURCES)
	@echo Compiling $@
	@mkdir -p build/
	@rustc src/main.rs $(LINK_FLAGS) -o $@

run: $(BINARIES)
	./build/compiler

check: build/test
	@./$<

build/test: src/test.rs $(WEB_SOURCES)
	@echo Compiling $@ in test mode
	@rustc $< $(LINK_FLAGS) $(RUST_FLAGS) --test --out-dir build/

clean:
	@echo "Cleaning ..."
	@rm -f build/* $(BINARIES)

.PHONY: check clean run
