use keystone::{Arch, Keystone, MODE_64};

fn main() {
    let engine = Keystone::new(Arch::X86, MODE_64).expect("Could not initialize Keystone engine");

    let code = String::from(/* Insert code here */ "");

    let encoding = engine.asm(code, 0)
        .expect("Could not assemble");

    let raw_bytes = encoding.bytes
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<String>();
    println!("Raw Bytes:\n{}", raw_bytes);

    let python_escaped = encoding.bytes
        .iter()
        .map(|b| format!("\\x{:02x}", b))
        .collect::<String>();
    println!("\nPython Escaped:\n\"{}\"", python_escaped);
}
