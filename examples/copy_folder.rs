use reflink_copy::{ReflinkOrCopyStatus, ReflinkSupport};
use std::fs;
use std::io;
use std::path::Path;
use walkdir::WalkDir;

// cargo run --example copy_folder V:/1 V:/2

fn main() -> io::Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <source_folder> <target_folder>", args[0]);
        return Ok(());
    }

    let from = Path::new(&args[1]);
    let to = Path::new(&args[2]);
    let reflink_support = reflink_copy::check_reflink_support(from, to)?;
    println!("Reflink support: {reflink_support:?}");

    let mut reflinked_count = 0u64;
    let mut copied_count = 0u64;
    let mut copied_data_size = 0u64;

    for entry in WalkDir::new(from) {
        let entry = entry?;
        let relative_path = entry.path().strip_prefix(from).unwrap();
        let target_path = to.join(relative_path);

        if entry.file_type().is_dir() {
            fs::create_dir_all(&target_path)?;
        } else {
            match reflink_support {
                ReflinkSupport::Supported => {
                    reflink_copy::reflink(entry.path(), target_path)?;
                    reflinked_count = reflinked_count.saturating_add(1);
                }
                ReflinkSupport::Unknown => {
                    let status = reflink_copy::reflink_or_copy(entry.path(), target_path)?;
                    match status {
                        ReflinkOrCopyStatus::Reflink => {
                            reflinked_count = reflinked_count.saturating_add(1);
                        }
                        ReflinkOrCopyStatus::Copy(file_size) => {
                            copied_count = copied_count.saturating_add(1);
                            copied_data_size = copied_data_size.saturating_add(file_size);
                        }
                    }
                }
                ReflinkSupport::NotSupported => {
                    let file_size = fs::copy(entry.path(), target_path)?;
                    copied_count = copied_count.saturating_add(1);
                    copied_data_size = copied_data_size.saturating_add(file_size);
                }
            }
        }
    }

    println!("reflinked files count: {reflinked_count}");
    println!("copied files count: {copied_count}");
    println!("copied data size: {copied_data_size} bytes");
    Ok(())
}
