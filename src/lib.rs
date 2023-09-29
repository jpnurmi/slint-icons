use std::collections::HashMap;
use std::path::PathBuf;

pub fn import_paths() -> HashMap<String, PathBuf> {
    HashMap::from([(
        env!("CARGO_PKG_NAME").to_string(),
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("ui"),
    )])
}
