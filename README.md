# `bl-rom-rt`

Minimal synchronous runtime with flash header for Bouffalo SoC's.

## Build an example

Make sure your Rust compiler have `riscv64imac-unknown-none-elf` target installed.

```
cargo build --example empty --target riscv64imac-unknown-none-elf
```

Check output ELF information:

```
riscv64-unknown-elf-objdump -d .\target\riscv64imac-unknown-none-elf\debug\examples\empty > target/output.S
```
