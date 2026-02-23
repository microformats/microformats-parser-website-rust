fn main() {
    let lock = std::fs::read_to_string("Cargo.lock").expect("Failed to read Cargo.lock");
    let mut found_microformats = false;

    for line in lock.lines() {
        if line.contains("name = \"microformats\"")
            && !line.contains("microformats-parser-website")
            && !line.contains("microformats-types")
        {
            found_microformats = true;
            continue;
        }
        if found_microformats && line.starts_with("version = ") {
            let version = line.split('"').nth(1).expect("Failed to parse version");
            println!("cargo:rustc-env=MF2_VERSION={}", version);
            break;
        }
    }

    if !found_microformats {
        panic!("Could not find microformats version in Cargo.lock");
    }

    println!("cargo:rerun-if-changed=Cargo.lock");
}
