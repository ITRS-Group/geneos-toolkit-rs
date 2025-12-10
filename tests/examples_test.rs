use pretty_assertions::assert_eq;
use std::fs;
use std::process::Command;
use std::str;

/// Helper function to run an example and test its output
fn test_example(name: &str, expected_output: &str) {
    println!("Testing example: {}", name);

    // Run the example
    let output = Command::new("cargo")
        .args(["run", "--example", name])
        .output()
        .unwrap_or_else(|_| panic!("Failed to execute example {}", name));

    assert!(output.status.success(), "Example {} failed to run", name);

    let stdout = str::from_utf8(&output.stdout).expect("Invalid UTF-8");

    // Check that the expected output matches stdout
    assert_eq!(stdout, expected_output);
}

/// Test all examples in the examples directory
#[test]
fn test_all_examples() {
    // Known examples and their expected outputs
    let examples = [
        (
            "basic_dataview",
            "\
Process,Status,CPU,Memory
<!>Example,Basic Dataview
process1,Running,2.5%,150MB
process2,Stopped,0.0%,0MB
",
        ),
        (
            "dataview_with_commas_in_cells",
            "\
Name,Age,Location
<!>Example,Dataview with Commas
Alice,30,Los Angeles\\, CA
Bob,25,New York\\, NY
Charlie,35,San Francisco\\, CA
",
        ),
        (
            "dataview_with_multiple_headlines",
            "\
Process,Status
<!>TotalProcesses,50
<!>TotalCache,300
<!>TotalMemory,1000
Process 1,OK
",
        ),
        (
            "files_iter",
            "\
file,kind,size_bytes
<!>example,files_iter
alpha.txt,file,1
beta.log,file,2
gamma.bin,file,3
",
        ),
        (
            "readme_iterative_rows",
            "\
host,status,cpu
<!>source,inventory
gamma,up,n/a
beta,up,n/a
alpha,up,n/a
",
        ),
        // ... Future examples should be added here as they are created
    ];

    for (name, expected) in examples {
        test_example(name, expected);
    }
}

/// Auto-discovers all examples and ensures they at least compile and run
#[test]
fn test_all_examples_compile() {
    // Get all .rs files in the examples directory
    if let Ok(entries) = fs::read_dir("examples") {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "rs") {
                if let Some(file_stem) = path.file_stem() {
                    if let Some(name) = file_stem.to_str() {
                        // Just check that the example compiles and runs without errors
                        let output = Command::new("cargo")
                            .args(["run", "--example", name])
                            .output()
                            .unwrap_or_else(|_| panic!("Failed to execute example {}", name));

                        assert!(
                            output.status.success(),
                            "Example {} failed to run. Error: {}",
                            name,
                            String::from_utf8_lossy(&output.stderr)
                        );
                    }
                }
            }
        }
    }
}
