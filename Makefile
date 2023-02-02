.PHONY: build

ARCH = x86_64-unknown-linux-musl
BIN_NAME = bootstrap

SAM_TEMPLATE = sam/template.yaml
ARTIFACTS_DIR = build
STACK_NAME = microservice-actix-sam
AWS_REGION = us-east-1

test:
	cargo test -- --nocapture

check:
	@cargo fmt --all
	@cargo clippy --no-deps -- -D warnings

serve: check
	fuser -k 7878/tcp || true && cargo run

coverage:
	cargo tarpaulin --ignore-tests --avoid-cfg-tarpaulin

audit:
	cargo audit

lint:
	cargo fmt --all --check && cargo clippy --no-deps -- -D warnings

build:
	cargo +nightly build --release --target $(ARCH)

package:
	rm -rf $(ARTIFACTS_DIR) && mkdir -p $(ARTIFACTS_DIR)
	cp -v ./target/$(ARCH)/release/$(BIN_NAME) ./$(ARTIFACTS_DIR)/bootstrap
	cp -v ./$(SAM_TEMPLATE) ./$(ARTIFACTS_DIR)/template.yaml

ci-package: package
	mkdir wrapper
	cp -v ./Makefile ./wrapper/Makefile
	mv $(ARTIFACTS_DIR) wrapper
	mv wrapper $(ARTIFACTS_DIR)

deploy:
	sam validate --template ./$(ARTIFACTS_DIR)/template.yaml
	sam deploy --template ./$(ARTIFACTS_DIR)/template.yaml \
	--stack-name $(STACK_NAME) \
	--region $(AWS_REGION) \
	--resolve-s3 \
	--capabilities CAPABILITY_IAM \
	--no-confirm-changeset \
	--no-fail-on-empty-changeset \
	--force-upload

destroy:
	sam delete \
	--stack-name $(STACK_NAME) \
	--region $(AWS_REGION) \
	--no-prompts