#![cfg(windows)]
#![cfg(test_with_dev_drive)]

use reflink_copy::{check_reflink_support, ReflinkSupport};
use std::path::{Path, PathBuf};

// paths are defined in build.yml

fn temp_dir() -> PathBuf {
    PathBuf::from(std::env::var("RUNNER_TEMP").expect("RUNNER_TEMP is not set"))
}
fn refs1_dir() -> PathBuf {
    temp_dir().join("dev-drives").join("refs1")
}
fn refs2_dir() -> PathBuf {
    temp_dir().join("dev-drives").join("refs2")
}
fn ntfs_dir() -> PathBuf {
    temp_dir().join("dev-drives").join("ntfs")
}

#[test]
fn test_reflink_support_refs1_to_refs2() {
    let result = check_reflink_support(refs1_dir(), refs2_dir()).unwrap();
    assert_eq!(result, ReflinkSupport::NotSupported);
}

#[test]
fn test_reflink_support_ntfs_to_refs1() {
    let result = check_reflink_support(ntfs_dir(), refs1_dir()).unwrap();
    assert_eq!(result, ReflinkSupport::NotSupported);
}

#[test]
fn test_reflink_support_refs1_to_ntfs() {
    let result = check_reflink_support(refs1_dir(), ntfs_dir()).unwrap();
    assert_eq!(result, ReflinkSupport::NotSupported);
}

#[test]
fn test_reflink_support_refs1() {
    let result = check_reflink_support(refs1_dir(), refs1_dir()).unwrap();
    assert_eq!(result, ReflinkSupport::Supported);

    let from = refs1_dir().join("subfolder1");
    let to = refs1_dir().join("subfolder2");
    let result = check_reflink_support(from, to).unwrap();
    assert_eq!(result, ReflinkSupport::Supported);
}
