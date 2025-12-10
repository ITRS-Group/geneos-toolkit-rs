use geneos_toolkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create rows using the new Row builder pattern
    let row1 = Row::new("server-01")
        .add_cell("cpu", "45%")
        .add_cell("memory", "2GB")
        .add_cell("status", "active");

    let row2 = Row::new("server-02")
        .add_cell("cpu", "12%")
        .add_cell("memory", "8GB")
        .add_cell("status", "idle");

    // Build the dataview
    let view = Dataview::builder()
        .set_row_header("hostname")
        .add_headline("region", "us-east-1")
        .add_row(row1)
        .add_row(row2)
        .build();

    print_result_and_exit(view);
}
