pub mod app;
pub mod data;
pub mod screens;
pub mod state;
pub mod ui_helpers;
pub mod widgets;

use crate::app::TikTokApp;
use fission::prelude::*;

#[cfg(target_os = "android")]
const ANDROID_TEST_CONTROL_PORT: u16 = 48761;

#[cfg(any(target_os = "android", target_os = "ios"))]
fn mobile_app() -> MobileApp<(), TikTokApp> {
    let app = MobileApp::<(), _>::new(TikTokApp::default()).with_title("TikTok Clone");
    #[cfg(target_os = "android")]
    let app = app.with_test_control_port(ANDROID_TEST_CONTROL_PORT);
    app
}

#[cfg(target_arch = "wasm32")]
fn web_app() -> WebApp<(), TikTokApp> {
    WebApp::<(), _>::new(TikTokApp::default()).with_title("TikTok Clone")
}

#[cfg(not(any(target_arch = "wasm32", target_os = "android", target_os = "ios")))]
pub fn run_desktop() {
    let _ = DesktopApp::<(), _>::new(TikTokApp::default()).run();
}

#[cfg(any(target_os = "android", target_os = "ios"))]
pub fn run_mobile() {
    let _ = mobile_app().run();
}

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app_handle: AndroidApp) {
    let _ = mobile_app().run_with_android_app(app_handle);
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn run_web() -> Result<(), wasm_bindgen::JsValue> {
    console_error_panic_hook::set_once();
    web_app()
        .run()
        .map_err(|error| wasm_bindgen::JsValue::from_str(&error.to_string()))
}
