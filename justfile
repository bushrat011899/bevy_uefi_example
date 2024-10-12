default:
    @just --list

alias b := build
alias r := run
alias c := check

# Run setup to ensure qemu and ovmf are available.
# Designed for Debian-like platforms with apt-get.
# For other platforms, refer to your package manager.
setup:
    apt-get install qemu ovmf

check:
    cargo check --target x86_64-unknown-uefi

build:
    cp /usr/share/OVMF/OVMF_CODE.fd .
    cp /usr/share/OVMF/OVMF_VARS.fd .
    mkdir -p esp/efi/boot
    cargo build --release --target x86_64-unknown-uefi
    cp target/x86_64-unknown-uefi/release/bevy_uefi_example.efi esp/efi/boot/

run:
    just --justfile {{justfile()}} build
    qemu-system-x86_64 \
        -drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd \
        -drive if=pflash,format=raw,readonly=on,file=OVMF_VARS.fd \
        -drive format=raw,file=fat:rw:esp