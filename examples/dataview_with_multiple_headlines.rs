use geneos_toolkit::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Create a new Dataview that contains multiple headlines in addition to the main
    // rows and columns.
    //
    // Note that the order of the headlines will be the same as the order in which they are
    // added to the Dataview.
    let view = Dataview::builder()
        .set_row_header("Process")
        .add_headline("TotalProcesses", 50)
        .add_headline("TotalCache", 300)
        .add_headline("TotalMemory", 1000)
        .add_value("Process 1", "Status", "OK")
        .build()?;

    println!("{}", view);
    Ok(())
}
