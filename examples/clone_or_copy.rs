// cargo run --example clone_or_copy V:\file.bin V:\file-clone.bin

use reflink_copy::ReflinkOrCopyStatus;

fn main() -> std::io::Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <source_file> <target_file>", args[0]);
        return Ok(());
    }
    let src_file = &args[1];
    let tgt_file = &args[2];

    let (status, _) = reflink_copy::reflink_or_copy(src_file, tgt_file)?;
    match status {
        ReflinkOrCopyStatus::Reflink => println!("File has been cloned"),
        ReflinkOrCopyStatus::Copy => println!("File has been copied"),
    }
    Ok(())
}
