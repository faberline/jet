// SPEC-MANAGED: .aw/tech-design/projects/jet/semantic/jet-tests.md#tests
// CODEGEN-BEGIN
//! End-to-end spike test: TSX → transpile → inspect output → verify
//! the emitted Rust contains the expected API calls.
//!
//! Structural checks explain lowering failures. The compiled counter test
//! also builds the emitted Rust against the pinned standalone Surface API
//! and runs its state/update behavior through jet-wasm.

use jet::tsx_to_rust::transpile;

const COUNTER_TSX: &str = include_str!("../fixtures/tsx_to_rust_counter.tsx");

#[test]
fn counter_transpiles_without_error() {
    let out = transpile(COUNTER_TSX).expect("transpile failed");
    // Show the generated code on stderr so --nocapture displays it.
    eprintln!("\n=== GENERATED ===\n{out}=================\n");
}

#[test]
fn generated_counter_compiles_and_updates_with_standalone_surface() {
    let root = tempfile::tempdir().unwrap();
    std::fs::create_dir(root.path().join("src")).unwrap();
    std::fs::copy(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.lock"),
        root.path().join("Cargo.lock"),
    )
    .unwrap();
    let runtime = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("wasm");
    std::fs::write(root.path().join("Cargo.toml"), format!(
        "[workspace]\n[package]\nname = \"surface-generated-counter\"\nversion = \"0.0.0\"\nedition = \"2021\"\n[dependencies]\njet-wasm = {{ path = {:?} }}\n", runtime.to_string_lossy()
    )).unwrap();
    let mut code = transpile(COUNTER_TSX).unwrap();
    code.push_str(
        r#"
#[test]
fn state_flows_through_the_generated_counter() {
    let mounted = jet_wasm::react::mount(counter(7));
    assert_eq!(mounted.snapshot().text_content(), "count: 7");
    mounted.snapshot().find_on_click("inc").unwrap().call(());
    assert_eq!(mounted.snapshot().text_content(), "count: 7");
    assert!(mounted.flush());
    assert_eq!(mounted.snapshot().text_content(), "count: 8");
    assert!(!mounted.flush());
}
"#,
    );
    std::fs::write(root.path().join("src/lib.rs"), code).unwrap();
    let output = std::process::Command::new("cargo")
        .args(["test", "--offline", "--manifest-path"])
        .arg(root.path().join("Cargo.toml"))
        .env("CARGO_BUILD_JOBS", "2")
        .env(
            "CARGO_TARGET_DIR",
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("target"),
        )
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "generated Rust failed:\n{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let lock = std::fs::read_to_string(root.path().join("Cargo.lock")).unwrap();
    assert_eq!(lock.matches("name = \"surface\"").count(), 1);
    assert_eq!(lock.matches("name = \"ui-runtime\"").count(), 1);
    assert!(lock.contains(
        "https://github.com/faberline/surface?rev=1754a2e96e487ea693be558041abdf21ffc63d27"
    ));
}

#[test]
fn generated_has_props_struct() {
    let out = transpile(COUNTER_TSX).unwrap();
    assert!(
        out.contains("pub struct CounterProps {"),
        "missing CounterProps struct.\nGENERATED:\n{out}"
    );
    assert!(
        out.contains("pub start: i64"),
        "missing typed `start` field.\nGENERATED:\n{out}"
    );
}

#[test]
fn generated_has_render_fn() {
    let out = transpile(COUNTER_TSX).unwrap();
    assert!(
        out.contains("fn counter_render(props:"),
        "missing render fn.\nGENERATED:\n{out}"
    );
    assert!(
        out.contains("props.downcast_ref"),
        "render fn should downcast props.\nGENERATED:\n{out}"
    );
}

#[test]
fn generated_calls_use_state_with_i64() {
    let out = transpile(COUNTER_TSX).unwrap();
    assert!(
        out.contains("use_state::<i64>(props.start)"),
        "useState binding should resolve `start` prop and turbofish i64.\nGENERATED:\n{out}"
    );
    assert!(
        out.contains("let (n, setN) = use_state"),
        "useState destructure should preserve names.\nGENERATED:\n{out}"
    );
}

#[test]
fn generated_emits_jsx_as_element_intrinsic() {
    let out = transpile(COUNTER_TSX).unwrap();
    assert!(
        out.contains("Element::intrinsic(\"button\""),
        "JSX button should lower to Element::intrinsic.\nGENERATED:\n{out}"
    );
    assert!(
        out.contains(".with_id("),
        "id attribute should land on Props.id.\nGENERATED:\n{out}"
    );
    assert!(
        out.contains(".with_on_click(Callback::new("),
        "onClick should lower to Callback::new.\nGENERATED:\n{out}"
    );
    assert!(
        out.contains("setN.set("),
        "setter call should route through StateSetter::set.\nGENERATED:\n{out}"
    );
}

#[test]
fn generated_emits_text_and_interpolation() {
    let out = transpile(COUNTER_TSX).unwrap();
    assert!(
        out.contains("Element::text(\"count: \")"),
        "static text should land as Element::text.\nGENERATED:\n{out}"
    );
    assert!(
        out.contains("Element::text(n.to_string())"),
        "{{n}} interpolation should lower to text content.\nGENERATED:\n{out}"
    );
}

#[test]
fn generated_has_factory_fn() {
    let out = transpile(COUNTER_TSX).unwrap();
    assert!(
        out.contains("pub fn counter(start: i64) -> Component {"),
        "factory fn signature missing.\nGENERATED:\n{out}"
    );
    assert!(
        out.contains("Rc::new(CounterProps { start })"),
        "factory should build CounterProps.\nGENERATED:\n{out}"
    );
}

#[test]
fn out_of_subset_fails_loudly() {
    let bad_tsx = "export class Thing extends Component {}";
    let err = transpile(bad_tsx).unwrap_err();
    let msg = format!("{err}");
    assert!(
        msg.contains("outside the spike subset") || msg.contains("outside the spike"),
        "unclear error: {msg}"
    );
    assert!(
        msg.contains(":"),
        "error should include source position: {msg}"
    );
}
// CODEGEN-END
