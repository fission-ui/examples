fn main() {
    #[cfg(not(any(target_arch = "wasm32", target_os = "android", target_os = "ios")))]
    tiktok_fission::run_desktop();
    
    #[cfg(any(target_os = "android", target_os = "ios"))]
    tiktok_fission::run_mobile();
}
