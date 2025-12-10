use geneos_toolkit::prelude::*;

struct Process {
    id: u32,
    name: String,
    cpu: f32,
    mem: u64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Simulate a collection of data (e.g., from an API or system call)
    let processes = vec![
        Process {
            id: 101,
            name: "nginx".to_string(),
            cpu: 1.2,
            mem: 1024,
        },
        Process {
            id: 102,
            name: "postgres".to_string(),
            cpu: 4.5,
            mem: 4096,
        },
        Process {
            id: 103,
            name: "redis".to_string(),
            cpu: 0.8,
            mem: 512,
        },
    ];

    // 2. Initialize the builder
    let mut builder = Dataview::builder()
        .set_row_header("pid")
        .add_headline("source", "system_monitor");

    // 3. Iterate over the data and add rows
    // Note: Since the builder methods consume 'self', we reassign 'builder' in each iteration
    for proc in processes {
        let row = Row::new(proc.id)
            .add_cell("name", proc.name)
            .add_cell("cpu", format!("{:.1}%", proc.cpu))
            .add_cell("memory", format!("{}MB", proc.mem));

        builder = builder.add_row(row);
    }

    // 4. Build and print
    print_result_and_exit(builder.build());
}
