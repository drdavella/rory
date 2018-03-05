CARGO = cargo

CARGO_OPTS =
BUILD_OPTS =

all:
	$(MAKE) build

build:
	$(CARGO) $(CARGO_OPTS) build $(BUILD_OPTS)

debug: BUILD_OPTS = "--features=debug"
debug: build


clean:
	$(CARGO) $(CARGO_OPTS) clean

check:
	$(MAKE) build
	$(MAKE) test

test:
	$(CARGO) $(CARGO_OPTS) test

bench:
	$(CARGO) $(CARGO_OPTS) bench

doc:
	$(CARGO) $(CARGO_OPTS) doc

.PHONY: all build clean check test bench doc
