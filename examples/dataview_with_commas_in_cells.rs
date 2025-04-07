use geneos_toolkit::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Create a new Dataview containing a list of people with their ages and locations
    // with commas in the location field.
    //
    // This example demonstrates how commas will be escaped in the dataview output.
    let dataview = Dataview::builder()
        .set_row_header("Name")
        .add_headline("Example", "Dataview with Commas")
        .add_value("Alice", "Age", "30")
        .add_value("Alice", "Location", "Los Angeles, CA")
        .add_value("Bob", "Age", "25")
        .add_value("Bob", "Location", "New York, NY")
        .add_value("Charlie", "Age", "35")
        .add_value("Charlie", "Location", "San Francisco, CA")
        .build()?;

    // Print the dataview to stdout
    println!("{}", dataview);
    Ok(())
}
