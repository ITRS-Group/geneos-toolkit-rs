use geneos_toolkit::prelude::*;

fn main() -> ! {
    // Create a simple dataview with process information
    let dataview = Dataview::builder()
        .set_row_header("Process")
        .add_headline("Example", "Basic Dataview")
        .add_value("process1", "Status", "Running")
        .add_value("process1", "CPU", "2.5%")
        .add_value("process1", "Memory", "150MB")
        .add_value("process2", "Status", "Stopped")
        .add_value("process2", "CPU", "0.0%")
        .add_value("process2", "Memory", "0MB")
        .build();

    print_result_and_exit(dataview)
}
