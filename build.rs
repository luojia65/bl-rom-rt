use std::{env, fs, path::PathBuf};

fn main() {
    let ld = &PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("link.x");
    fs::write(ld, LINK).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=LOG");
    println!("cargo:rustc-link-arg=-T{}", ld.display());
}

const LINK: &[u8] = b"
OUTPUT_ARCH(riscv)
ENTRY(main)
PROVIDE(stext = 0x80200000);

SECTIONS
{
    .text stext : {
        stext = .;
        *(.text.entry)
        *(.text .text.*)
        . = ALIGN(4);
        etext = .;
    }

    .rodata : ALIGN(4) {
        srodata = .;
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
        . = ALIGN(4);
        erodata = .;
    }

    .data : ALIGN(4) {
        sidata = LOADADDR(.data);
        sdata = .;
        PROVIDE(__global_pointer$ = . + 0x800);
        *(.data .data.*)
        *(.sdata .sdata.*)
        . = ALIGN(4);
        edata = .;
    }

    .bss (NOLOAD) : ALIGN(4) {
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        . = ALIGN(4);
        ebss = .;
    }

    /DISCARD/ : {
        *(.eh_frame .eh_frame_hdr)
    }
}";
