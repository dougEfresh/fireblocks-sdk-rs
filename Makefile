CARGO_TARGET_DIR ?= $(PWD)/target
OUT := $(CARGO_TARGET_DIR)/generated

$(OUT):
	mkdir $(OUT)

# https://raw.githubusercontent.com/fireblocks/fireblocks-openapi-spec/refs/heads/main/open_api_spec.yml
generate: $(OUT)
	rm -rf $(OUT) && cd generator && openapi-generator-cli generate -i ./open_api_spec.yml -o $(OUT) -g rust -c config.yaml 
