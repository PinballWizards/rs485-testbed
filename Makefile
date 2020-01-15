all:
	cargo build --release
	arm-none-eabi-objcopy -O binary target/thumbv6m-none-eabi/release/rs485-testbed target/thumbv6m-none-eabi/release/rs485-testbed.bin
	uf2conv-rs target/thumbv6m-none-eabi/release/rs485-testbed.bin