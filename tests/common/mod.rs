use std::fs;
use std::path::PathBuf;

pub fn json_fixtures(endpoint: &str) -> Vec<(PathBuf, Vec<u8>)> {
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(endpoint);

    let mut paths = fs::read_dir(&directory)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", directory.display()))
        .map(|entry| {
            entry
                .unwrap_or_else(|error| {
                    panic!(
                        "failed to read an entry in {}: {error}",
                        directory.display()
                    )
                })
                .path()
        })
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "json")
        })
        .collect::<Vec<_>>();

    paths.sort();

    assert!(
        !paths.is_empty(),
        "no JSON fixtures found in {}",
        directory.display()
    );

    paths
        .into_iter()
        .map(|path| {
            let body = fs::read(&path)
                .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
            (path, body)
        })
        .collect()
}
