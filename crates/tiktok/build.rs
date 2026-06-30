use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=assets/theme/dsp.json");
    println!("cargo:rerun-if-changed=assets/theme/tokens.json");

    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let dsp_path = manifest_dir.join("assets/theme/dsp.json");

    fission_design_system_codegen::generate(fission_design_system_codegen::Config {
        dsp_path,
        out_file: "tiktok_design_system.rs".into(),
        type_name: "TikTokDesignSystem".into(),
        crate_path: "fission::theme".into(),
    })
    .expect("failed to generate TikTokDesignSystem from assets/theme/dsp.json");
}
