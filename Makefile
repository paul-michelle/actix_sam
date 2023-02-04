.PHONY: build

ARCH = x86_64-unknown-linux-musl
BIN_NAME = bootstrap

SAM_TEMPLATE = template.yaml
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
	rm -rf build && mkdir -p build
	cp -v ./target/$(ARCH)/release/$(BIN_NAME) build/bootstrap

ci-package: package
	mkdir interim-wrapper
	cp -v ./$(SAM_TEMPLATE) ./interim-wrapper/template.yaml
	cp -v ./Makefile ./interim-wrapper/Makefile
	mv build interim-wrapper
	mv interim-wrapper build

deploy:
	sam validate
	sam deploy \
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