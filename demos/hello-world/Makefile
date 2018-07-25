WORKSPACE_DIR=../

APP_NAME="Demo"
APP_VERSION=0.0.0
APP_ICON_HEX=0100000000ffffff00ffffffffffffffffffff1ffe9ffc9ffc1ffe9ffd9ff9ffffffffffffffffffff

NANOS_TARGET_ID=0x31100003
RELEASE_ELF=$(WORKSPACE_DIR)target/thumbv6m-none-eabi/release/hello-world
RELEASE_HEX=$(RELEASE_ELF).hex

PRIVATE_KEY=$(shell cat $(WORKSPACE_DIR)customCA.key)
ifneq ($(PRIVATE_KEY),)
PARAM_PRIVATE_KEY=--rootPrivateKey $(PRIVATE_KEY)
endif

.PHONY: load
load:
	cargo build --release
	arm-none-eabi-objcopy -O ihex -S $(RELEASE_ELF) $(RELEASE_HEX)
	python -m ledgerblue.loadApp \
		$(PARAM_PRIVATE_KEY) \
		--targetId $(NANOS_TARGET_ID) \
		--tlv --delete \
		--fileName $(RELEASE_HEX) \
		--appName $(APP_NAME) \
		--appVersion $(APP_VERSION) \
		--dataSize 0 \
		--icon $(APP_ICON_HEX)

.PHONY: delete
delete:
	python -m ledgerblue.deleteApp \
		$(PARAM_PRIVATE_KEY) \
		--targetId $(NANOS_TARGET_ID) \
		--appName $(APP_NAME)