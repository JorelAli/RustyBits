use embed_manifest::embed_manifest_file;

fn main() {
    embed_manifest_file("days_until_christmas.exe.manifest")
        .expect("unable to embed manifest file");
    println!("cargo:rerun-if-changed=days_until_christmas.exe.manifest");
}