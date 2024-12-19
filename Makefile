CARGO_TARGET_DIR ?= $(PWD)/target
OUT := $(CARGO_TARGET_DIR)/generated

$(OUT):
	mkdir $(OUT)

# https://raw.githubusercontent.com/fireblocks/fireblocks-openapi-spec/refs/heads/main/api-spec-v2.yaml
generate: $(OUT)
	rm -rf $(OUT) && cd generator && openapi-generator-cli generate -i fireblocks-public-openapi.yaml -o $(OUT) -g rust -c config.yaml  --skip-validate-spec
