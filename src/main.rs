use anyhow::Result;
use env_logger::Builder;
use log::info;
use log::LevelFilter;
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
use tempfile::NamedTempFile;

fn prepare_test_data() {
    let app_data_dir = "app_data";
    let input_file = "app_data/placeholder_for_getting_parent_folder.txt";
    let output_dir = "app_data/target";
    let output_file = "app_data/target/out.txt";

    // Create app_data directory
    if !Path::new(app_data_dir).exists() {
        fs::create_dir(app_data_dir).expect("Failed to create app_data directory");
    }

    // Create placeholder file for getting parent folder
    if !Path::new(input_file).exists() {
        let mut file = fs::File::create(input_file)
            .expect("Failed to create placeholder_for_getting_parent_folder.txt");
        writeln!(file, "placeholder_for_getting_parent_folder")
            .expect("Failed to write to placeholder_for_getting_parent_folder.txt");
    }

    // Create target directory
    if !Path::new(output_dir).exists() {
        fs::create_dir_all(output_dir).expect("Failed to create target directory");
    }

    // Clean existing output file
    if Path::new(output_file).exists() {
        fs::remove_file(output_file).expect("Failed to remove existing out.txt");
    }
}

fn main() -> Result<()> {
    Builder::new().filter_level(LevelFilter::Info).init();

    prepare_test_data();

    let args: Vec<_> = env::args().collect();
    info!("Arguments: {:?}", args);

    if args.len() < 3 {
        eprintln!("Usage: {} <existing_file_path> <output_file_path>", args[0]);
        std::process::exit(1);
    }

    let a = &args[1];
    let b = &args[2];

    info!(
        "Using '{}' to determine the parent directory for the temporary file.",
        a
    );
    let parent = Path::new(a).parent().unwrap_or_else(|| Path::new("."));
    info!("Parent directory for temporary file: {:?}", parent);

    let mut tmp = NamedTempFile::new_in(parent)?;

    info!("Temporary file created at: {:?}", tmp.path());
    // todo linux test (as in zed)
    tmp.write_all("".as_bytes())?;
    // works
    // write!(tmp, "")?;
    info!("Trying to persist to path: {}", b);
    tmp.persist(b)?;

    info!("Temporary file persisted to: {}", b);

    Ok(())
}
