#![cfg(windows)]

use reflink_copy::{check_reflink_support, ReflinkSupport};
use std::path::Path;

// paths are defined in build.yml
const REFS1: &str = "./dev-drives/refs1";
const REFS2: &str = "./dev-drives/refs2";
const NTFS: &str = "./dev-drives/ntfs";

#[test]
fn test_reflink_support_refs1_to_refs2() {
    let result = check_reflink_support(REFS1, REFS2).unwrap();
    assert_eq!(result, ReflinkSupport::NotSupported);
}

#[test]
fn test_reflink_support_ntfs_to_refs1() {
    let result = check_reflink_support(NTFS, REFS1).unwrap();
    assert_eq!(result, ReflinkSupport::NotSupported);
}

#[test]
fn test_reflink_support_refs1_to_ntfs() {
    let result = check_reflink_support(REFS1, NTFS).unwrap();
    assert_eq!(result, ReflinkSupport::NotSupported);
}

#[test]
fn test_reflink_support_refs1() {
    let result = check_reflink_support(REFS1, REFS1).unwrap();
    assert_eq!(result, ReflinkSupport::Supported);

    let from = Path::new(REFS1).join("subfolder1");
    let to = Path::new(REFS1).join("subfolder2");
    let result = check_reflink_support(from, to).unwrap();
    assert_eq!(result, ReflinkSupport::Supported);
}
