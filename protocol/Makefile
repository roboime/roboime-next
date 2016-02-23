PROTO_FILES := $(shell find proto -name "*.proto")
RUST_FILES := $(patsubst proto/%.proto,src/%.rs,$(PROTO_FILES))

.PHONY: all
all: $(RUST_FILES)

.PHONY: clean
clean:
	rm -rf $(RUST_FILES)

src/%.rs: proto/%.proto
	protoc --proto_path proto --rust_out src $<
