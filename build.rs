use std::process::Command;

fn main() {
    for output in Command::new("net-snmp-config").arg("--libdir").output() {
        if output.status.success() {
            for path in String::from_utf8(output.stdout) {
                if path.trim().len() > 0 {
                    println!("cargo:rustc-link-search=native={}", &path);
                }
            }
        }
    }

    println!("cargo:rustc-link-lib=dylib=pci");
    println!("cargo:rustc-link-lib=dylib=dl");
    println!("cargo:rustc-link-lib=dylib=sensors");
    println!("cargo:rustc-link-lib=dylib=wrap");
    println!("cargo:rustc-link-lib=dylib=m");
    println!("cargo:rustc-link-lib=dylib=netsnmp");
    println!("cargo:rustc-link-lib=dylib=netsnmpagent");
    println!("cargo:rustc-link-lib=dylib=crypto");
}
