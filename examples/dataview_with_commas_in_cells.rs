use geneos_toolkit::prelude::*;
use std::error::Error;

// Name,Age,Location
// <!>Example,Dataview with Commas
// Alice,30,Los Angeles\\, CA
// Bob,25,New York\\, NY
// Charlie,35,San Francisco\\, CA
fn main() -> Result<(), Box<dyn Error>> {
    // Create a simple dataview with process information
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
