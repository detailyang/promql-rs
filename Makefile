# Background color
GREEN    := $(shell tput -Txterm setaf 2)
YELLOW   := $(shell tput -Txterm setaf 3)
BLUE     := $(shell tput -Txterm setaf 4)
MAGENTA  := $(shell tput -Txterm setaf 5)
WHITE    := $(shell tput -Txterm setaf 7)
RESET    := $(shell tput -Txterm sgr0)
PADDING  := $(shell printf '%-24s' "")

TARGET_MAX_CHAR_NUM := 20

## Show help
help:
	@echo ''
	@echo 'Usage:'
	@echo '  ${YELLOW}make${RESET} ${GREEN}<target>${RESET} ${MAGENTA}[variable=value]${RESET}'
	@echo ''
	@echo 'Targets:'
	@awk '/^[a-zA-Z\-\_0-9]+:/ { \
		helpMessage = match(lastLine, /^## (.*)/); \
		if (helpMessage) { \
			helpCommand = substr($$1, 0, index($$1, ":")-1); \
			helpMessage = substr(lastLine, RSTART + 3, RLENGTH); \
			gsub(/;;/,"\n${PADDING}${MAGENTA}>variable: ${RESET}", helpMessage); \
			printf "  ${YELLOW}%-$(TARGET_MAX_CHAR_NUM)s${RESET} ${GREEN}%s${RESET}\n", helpCommand, helpMessage; \
		} \
	} \
	{ lastLine = $$0 }' $(MAKEFILE_LIST)

SHELL         := /bin/bash

.PHONY: lint
## Run cargo clippy
lint:
	cargo clippy

.PHONY: format
## Run cargo check
format:
	cargo fmt

.PHONY: test
## Run cargo test
test:
	cargo test

