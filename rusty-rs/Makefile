APP_NAME="Rusty"
APP_VERSION=0.1.0
APP_ICON_HEX=0100000000ffffff00ffffffffffffffffffff1ffe9ffc9ffc1ffe9ffd9ff9ffffffffffffffffffff

NANOS_TARGET_ID=0x31100003
RELEASE_ELF=target/thumbv6m-none-eabi/release/rusty-rs
RELEASE_HEX=$(RELEASE_ELF).hex

load:
	cargo build --release
	arm-none-eabi-objcopy -O ihex -S $(RELEASE_ELF) $(RELEASE_HEX)
	python -m ledgerblue.loadApp \
		--rootPrivateKey `cat ../customCA.key` \
		--targetId $(NANOS_TARGET_ID) \
		--tlv --delete \
		--fileName $(RELEASE_HEX) \
		--appName $(APP_NAME) \
		--appVersion $(APP_VERSION) \
		--dataSize 0 \
		--icon $(APP_ICON_HEX)