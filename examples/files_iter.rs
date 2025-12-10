use geneos_toolkit::prelude::*;
use std::fs;
use tempfile::tempdir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a temporary directory with some files so the example is deterministic.
    let temp = tempdir()?;
    let dir_path = temp.path();

    fs::write(dir_path.join("alpha.txt"), b"a")?;
    fs::write(dir_path.join("beta.log"), b"bb")?;
    fs::write(dir_path.join("gamma.bin"), b"ccc")?;

    // Gather directory entries and sort to ensure stable output order.
    let mut entries: Vec<_> = fs::read_dir(dir_path)?.filter_map(|e| e.ok()).collect();
    entries.sort_by_key(|e| e.file_name());

    // Build the dataview using the Row builder.
    let mut builder = Dataview::builder()
        .set_row_header("file")
        .add_headline("example", "files_iter");

    for entry in entries {
        let meta = entry.metadata()?;
        let name = entry.file_name().to_string_lossy().into_owned();
        let size_bytes = meta.len();
        let kind = if meta.is_dir() {
            "dir"
        } else if meta.is_file() {
            "file"
        } else {
            "other"
        };

        let row = Row::new(name)
            .add_cell("kind", kind)
            .add_cell("size_bytes", size_bytes.to_string());

        builder = builder.add_row(row);
    }

    print_result_and_exit(builder.build());
}
