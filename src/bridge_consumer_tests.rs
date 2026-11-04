//! Consumer acceptance retained when browser automation moved to jet-bridge.
use jet_bridge::browser::locator::safe_locator_json_str;
use jet_bridge::browser_cli::format_browser_cli_ctrl_c_warn;
fn sample_err() -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::PermissionDenied, "EPERM browser sample")
}
#[test]
fn gh3737_safe_locator_agrees_with_safe_define_for_same_inputs() {
    use crate::bundler::define::safe_import_meta_env_define_value;
    for case in ["", "x", "a\"b", "multi\nline", "\u{0001}", "ßéü"] {
        assert_eq!(
            safe_locator_json_str(case),
            safe_import_meta_env_define_value(case),
            "the two defensive escapers must agree for input {case:?}"
        );
    }
}

#[test]
fn helper_distinct_from_dev_server_3725_and_wasm_dev_3730_warns() {
    let me = format_browser_cli_ctrl_c_warn(&sample_err());
    let dev = crate::dev_server::format_dev_server_ctrl_c_warn(&sample_err());
    let wasm = crate::wasm_dev::format_wasm_dev_ctrl_c_warn(&sample_err());
    assert_ne!(me, dev);
    assert_ne!(me, wasm);
    // Sibling warns must not pose as us (no #3732 in their bodies
    // as the primary issue tag).
    assert!(
        !dev.contains("GH #3732"),
        "dev_server warn must not carry our tag: {dev}"
    );
    assert!(
        !wasm.contains("GH #3732"),
        "wasm_dev warn must not carry our tag: {wasm}"
    );
}
