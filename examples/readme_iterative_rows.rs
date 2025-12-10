use geneos_toolkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Unknown order at runtime; sorted descending by name afterward.
    let hosts = ["beta", "alpha", "gamma"];

    let mut builder = Dataview::builder()
        .set_row_header("host")
        .add_headline("source", "inventory");

    for name in hosts {
        let row = Row::new(name)
            .add_cell("status", "up")
            .add_cell("cpu", "n/a");
        builder = builder.add_row(row);
    }

    let dataview = builder
        .sort_rows_with(|a, b| b.cmp(a))
        .build()?;

    print_result_and_exit(Ok(dataview))
}
