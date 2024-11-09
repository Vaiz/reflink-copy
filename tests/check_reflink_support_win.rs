#![cfg(windows)]

use reflink_copy::{check_reflink_support, reflink, reflink_or_copy, ReflinkSupport};
use std::io::Write;
use std::path::{Path, PathBuf};

const FILE_SIZE: usize = 256 * 1024;
const FILENAME: &str = "test_file.dat";

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

fn subfolder_path(folder: &Path, line: u32) -> PathBuf {
    folder.join(format!("subfolder_{line}"))
}

fn create_test_file(path: &Path) -> std::io::Result<()> {
    if let Some(folder) = path.parent() {
        std::fs::create_dir_all(folder)?;
    }

    let mut file = std::fs::File::create(path)?;
    file.write_all(&vec![0u8; FILE_SIZE])?;
    Ok(())
}

#[test]
#[ignore]
fn test_correct_deployment() {
    assert!(temp_dir().join("dev-drives").join("ntfs.vhdx").exists());
}

#[test]
#[ignore]
fn test_reflink_support_refs1_to_refs2() {
    let result = check_reflink_support(refs1_dir(), refs2_dir()).unwrap();
    assert_eq!(result, ReflinkSupport::NotSupported);

    let from = subfolder_path(&refs1_dir(), line!());
    let to = subfolder_path(&refs2_dir(), line!());
    let result = check_reflink_support(from, to).unwrap();
    assert_eq!(result, ReflinkSupport::NotSupported);
}

#[test]
#[ignore]
fn test_reflink_support_ntfs_to_refs1() {
    let result = check_reflink_support(ntfs_dir(), refs1_dir()).unwrap();
    assert_eq!(result, ReflinkSupport::NotSupported);

    let from = subfolder_path(&ntfs_dir(), line!());
    let to = subfolder_path(&refs1_dir(), line!());
    let result = check_reflink_support(from, to).unwrap();
    assert_eq!(result, ReflinkSupport::NotSupported);
}

#[test]
#[ignore]
fn test_reflink_support_refs1_to_ntfs() {
    let result = check_reflink_support(refs1_dir(), ntfs_dir()).unwrap();
    assert_eq!(result, ReflinkSupport::NotSupported);

    let from = subfolder_path(&refs1_dir(), line!());
    let to = subfolder_path(&ntfs_dir(), line!());
    let result = check_reflink_support(from, to).unwrap();
    assert_eq!(result, ReflinkSupport::NotSupported);
}

#[test]
#[ignore]
fn test_reflink_support_refs1() {
    let result = check_reflink_support(refs1_dir(), refs1_dir()).unwrap();
    assert_eq!(result, ReflinkSupport::Supported);

    let from = subfolder_path(&refs1_dir(), line!());
    let to = subfolder_path(&refs1_dir(), line!());
    let result = check_reflink_support(from, to).unwrap();
    assert_eq!(result, ReflinkSupport::Supported);
}

#[test]
#[ignore]
fn test_reflink_on_supported_config() -> std::io::Result<()> {
    let from = subfolder_path(&refs1_dir(), line!());
    let to = subfolder_path(&refs1_dir(), line!());
    create_test_file(&from.join(FILENAME))?;
    reflink(from.join(FILENAME), to.join(FILENAME))
}

#[test]
#[ignore]
fn test_reflink_on_unsupported_config() -> std::io::Result<()> {
    let from = subfolder_path(&refs1_dir(), line!());
    let to = subfolder_path(&refs2_dir(), line!());
    create_test_file(&from.join(FILENAME))?;
    let result = reflink(from.join(FILENAME), to.join(FILENAME)).unwrap_err();
    assert_eq!(result.to_string(), "Incorrect function.");
    Ok(())
}


#[test]
#[ignore]
fn test_reflink_or_copy_on_supported_config() -> std::io::Result<()> {
    let from = subfolder_path(&refs1_dir(), line!());
    let to = subfolder_path(&refs1_dir(), line!());
    create_test_file(&from.join(FILENAME))?;
    let result = reflink_or_copy(from.join(FILENAME), to.join(FILENAME))?;
    assert_eq!(result, None);
    Ok(())
}


#[test]
#[ignore]
fn test_reflink_or_copy_on_unsupported_config() -> std::io::Result<()> {
    let from = subfolder_path(&refs1_dir(), line!());
    let to = subfolder_path(&refs1_dir(), line!());
    create_test_file(&from.join(FILENAME))?;
    let result = reflink_or_copy(from.join(FILENAME), to.join(FILENAME))?;
    assert_eq!(result, Some(FILE_SIZE as u64));
    Ok(())
}
