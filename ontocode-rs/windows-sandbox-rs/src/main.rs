#[cfg(target_os = "windows")]
fn main() {
    panic!("ontocode-windows-sandbox is not a standalone executable");
}

#[cfg(not(target_os = "windows"))]
fn main() {
    panic!("ontocode-windows-sandbox is Windows-only");
}
