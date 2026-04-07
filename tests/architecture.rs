//! Structural tests that verify ARCHITECTURE.md module dependency rules hold in code.
//!
//! These tests parse `use crate::` statements in source files and assert that
//! no backward dependencies exist. If a test fails, it means someone introduced
//! an import that violates the dependency graph documented in ARCHITECTURE.md.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn collect_crate_imports(file_path: &Path) -> Vec<String> {
    let content = fs::read_to_string(file_path)
        .unwrap_or_else(|_| panic!("Could not read {}", file_path.display()));
    content
        .lines()
        .filter(|line| {
            let trimmed = line.trim();
            trimmed.starts_with("use crate::")
        })
        .map(|line| line.trim().to_string())
        .collect()
}

fn imports_module(imports: &[String], module_name: &str) -> bool {
    imports
        .iter()
        .any(|line| line.contains(&format!("crate::{module_name}")))
}

fn src_path() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

#[test]
fn detect_is_only_used_by_init() {
    let src = src_path();
    let command_modules = [
        "check.rs",
        "plan.rs",
        "sprint.rs",
        "status.rs",
        "gc.rs",
        "score.rs",
        "upgrade.rs",
        "cli.rs",
    ];

    for module in &command_modules {
        let path = src.join(module);
        if path.exists() {
            let imports = collect_crate_imports(&path);
            assert!(
                !imports_module(&imports, "detect"),
                "ARCHITECTURE VIOLATION: {module} imports from detect.rs, \
                 but detect.rs should only be used by init/. \
                 See ARCHITECTURE.md 'Module Dependency Rules'."
            );
        }
    }
}

#[test]
fn no_command_module_imports_cli() {
    let src = src_path();
    let command_modules = [
        "check.rs",
        "plan.rs",
        "sprint.rs",
        "status.rs",
        "gc.rs",
        "score.rs",
        "upgrade.rs",
        "config.rs",
        "detect.rs",
        "types.rs",
        "init/mod.rs",
        "init/render.rs",
    ];

    for module in &command_modules {
        let path = src.join(module);
        if path.exists() {
            let imports = collect_crate_imports(&path);
            assert!(
                !imports_module(&imports, "cli"),
                "ARCHITECTURE VIOLATION: {module} imports from cli.rs. \
                 Dependencies flow downward: cli.rs dispatches to command modules, \
                 not the other way around. See ARCHITECTURE.md 'Module Dependency Rules'."
            );
        }
    }
}

#[test]
fn command_modules_do_not_cross_import() {
    let src = src_path();

    // Each command module and what it is NOT allowed to import
    let forbidden: HashMap<&str, Vec<&str>> = HashMap::from([
        (
            "check.rs",
            vec!["plan", "sprint", "gc", "score", "upgrade", "status"],
        ),
        (
            "plan.rs",
            vec!["sprint", "check", "gc", "score", "upgrade", "status"],
        ),
        ("sprint.rs", vec!["plan", "check", "gc", "score", "upgrade"]),
        (
            "gc.rs",
            vec![
                "plan", "sprint", "check", "score", "upgrade", "status", "init",
            ],
        ),
        (
            "score.rs",
            vec!["plan", "sprint", "check", "gc", "upgrade", "status", "init"],
        ),
        // status.rs is allowed to import sprint (for SprintState) per ARCHITECTURE.md
        (
            "status.rs",
            vec!["plan", "check", "gc", "score", "upgrade", "init"],
        ),
        // upgrade.rs is allowed to import init/render per ARCHITECTURE.md
        (
            "upgrade.rs",
            vec!["plan", "sprint", "check", "gc", "score", "status"],
        ),
    ]);

    for (module, disallowed) in &forbidden {
        let path = src.join(module);
        if path.exists() {
            let imports = collect_crate_imports(&path);
            for dep in disallowed {
                assert!(
                    !imports_module(&imports, dep),
                    "ARCHITECTURE VIOLATION: {module} imports from {dep}. \
                     Command modules should not cross-import each other \
                     (except explicitly allowed edges in ARCHITECTURE.md). \
                     See ARCHITECTURE.md 'Module Dependency Rules'."
                );
            }
        }
    }
}

#[test]
fn config_does_not_import_command_modules() {
    let src = src_path();
    let path = src.join("config.rs");
    let imports = collect_crate_imports(&path);

    let command_modules = [
        "cli", "check", "plan", "sprint", "status", "gc", "score", "upgrade", "init", "detect",
    ];

    for module in &command_modules {
        assert!(
            !imports_module(&imports, module),
            "ARCHITECTURE VIOLATION: config.rs imports from {module}. \
             config.rs is a shared leaf dependency — it must not import command modules. \
             See ARCHITECTURE.md 'Module Dependency Rules'."
        );
    }
}

#[test]
fn types_does_not_import_any_crate_module() {
    let src = src_path();
    let path = src.join("types.rs");
    let imports = collect_crate_imports(&path);

    let any_internal = imports.iter().any(|line| line.starts_with("use crate::"));
    assert!(
        !any_internal,
        "ARCHITECTURE VIOLATION: types.rs imports from another crate module. \
         types.rs defines the newtype vocabulary and must be a pure leaf. \
         See ARCHITECTURE.md 'Module Dependency Rules'."
    );
}
