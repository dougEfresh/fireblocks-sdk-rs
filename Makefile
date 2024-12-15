CARGO_TARGET_DIR ?= $(PWD)/target
OUT := $(CARGO_TARGET_DIR)/generated

$(OUT):
	mkdir $(OUT)

generate: $(OUT)
	rm -rf $(OUT) && cd generator && openapi-generator-cli generate -i ./open_api_spec.yml -o $(OUT) -g rust -c config.yaml 
