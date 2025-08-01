fn main() {
    let o = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let cli_snapshot_path = o.join("HOMUNCULUS_SNAPSHOT.bin");
    create_cli_snapshot(cli_snapshot_path);
}

fn create_cli_snapshot(snapshot_path: std::path::PathBuf) {
    use deno_runtime::ops::bootstrap::SnapshotOptions;
    let snapshot_options = SnapshotOptions {
        target: std::env::var("TARGET").unwrap(),
        ..Default::default()
    };
    deno_runtime::snapshot::create_runtime_snapshot(snapshot_path, snapshot_options, vec![]);
}
