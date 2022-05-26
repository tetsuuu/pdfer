IMAGE_TAG         := $(shell git rev-parse --short HEAD)
MODULE_NAME       := pdfer
IMAGE_BASE_NAME   := local
IMAGE_NAME        := $(IMAGE_BASE_NAME)-$(MODULE_NAME):$(IMAGE_TAG)
IMAGE_LATEST_NAME := $(IMAGE_BASE_NAME)-$(MODULE_NAME):latest

.PHONY: docker-build
docker-build:
	docker build -f ./local/app/Dockerfile -t $(IMAGE_NAME) .
	docker build -f ./local/app/Dockerfile -t $(IMAGE_LATEST_NAME) .

.PHONY: build
build:
	cargo build

.PHONY: release-build
release-build:
	cargo build --release

.PHONY: audit
audit:
	cargo audit

.PHONY: test
test:
	ccargo test --locked
