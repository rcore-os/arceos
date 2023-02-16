use std::io::Result;

fn main() {
    let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    gen_linker_script(&arch).unwrap();
}

fn gen_linker_script(arch: &str) -> Result<()> {
    let ld_content = std::fs::read_to_string("linker.lds.S")?;
    let ld_content = ld_content.replace("%ARCH%", arch).replace(
        "%KERNEL_BASE%",
        &format!("{:#x}", axconfig::KERNEL_BASE_VADDR),
    );
    #[cfg(feature = "platform-qemu-virt-aarch64")]
    let ld_content = ld_content.replace("sdata = .;", "sdata = .;\n\t\t*(.data.boot_page_table)");
    std::fs::write("linker.lds", ld_content)?;
    Ok(())
}
