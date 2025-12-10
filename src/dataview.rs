use std::collections::HashMap;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum DataviewError {
    MissingRowHeader,
    MissingValue,
}

impl fmt::Display for DataviewError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataviewError::MissingRowHeader => write!(f, "The Dataview must have a row header"),
            DataviewError::MissingValue => write!(f, "The Dataview must have at least one value"),
        }
    }
}

impl Error for DataviewError {}

/// A Geneos Dataview object.
///
/// This struct represents a Dataview, which is a structured representation of data
/// with a row header, headlines, and values.
///
/// Example Dataview format:
/// ```text
/// row_header,column1,column2
/// <!>headline1,value1
/// <!>headline2,value2
/// row1,value1,value2
/// row2,value1,value2
/// ```
///
/// Example with data:
/// ```text
/// cpu,percentUtilisation,percentIdle
/// <!>numOnlineCpus,2
/// <!>loadAverage1Min,0.32
/// <!>loadAverage5Min,0.45
/// <!>loadAverage15Min,0.38
/// <!>HyperThreadingStatus,ENABLED
/// Average_cpu,3.75 %,96.25 %
/// cpu_0,3.25 %,96.75 %
/// cpu_0_logical#1,2.54 %,97.46 %
/// cpu_0_logical#2,2.54 %,97.46 %
/// ```
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Dataview {
    row_header: String,
    headlines: HashMap<String, String>,
    headline_order: Vec<String>,
    values: HashMap<(String, String), String>,
    column_order: Vec<String>,
    row_order: Vec<String>,
}

impl Dataview {
    /// Returns the row header label for this dataview.
    ///
    /// # Example
    /// ```
    /// use geneos_toolkit::dataview::DataviewBuilder;
    /// let view = DataviewBuilder::new()
    ///     .set_row_header("Process")
    ///     .add_value("proc1", "Status", "Running")
    ///     .build()
    ///     .unwrap();
    /// assert_eq!(view.row_header(), "Process");
    /// ```
    pub fn row_header(&self) -> &str {
        &self.row_header
    }

    /// Returns a headline value by key, if present.
    pub fn headline(&self, key: &str) -> Option<&String> {
        self.headlines.get(key)
    }

    /// Returns the headline keys in display order.
    pub fn headline_order(&self) -> &[String] {
        &self.headline_order
    }

    /// Returns a cell value for the given row/column, if present.
    pub fn value(&self, row: &str, column: &str) -> Option<&String> {
        self.values.get(&(row.to_string(), column.to_string()))
    }

    /// Returns the column names in display order.
    pub fn column_order(&self) -> &[String] {
        &self.column_order
    }

    /// Returns the row names in display order.
    pub fn row_order(&self) -> &[String] {
        &self.row_order
    }
}

fn escape_commas(s: &str) -> String {
    s.replace(",", "\\,")
}

fn write_header_row(
    f: &mut fmt::Formatter<'_>,
    row_header: &str,
    columns: &[String],
) -> fmt::Result {
    write!(f, "{}", escape_commas(row_header))?;
    for col in columns {
        write!(f, ",{}", escape_commas(col))?;
    }
    writeln!(f)
}

fn write_headlines(
    f: &mut fmt::Formatter<'_>,
    headline_order: &[String],
    headlines: &HashMap<String, String>,
) -> fmt::Result {
    for name in headline_order {
        if let Some(value) = headlines.get(name) {
            writeln!(f, "<!>{},{}", escape_commas(name), escape_commas(value))?;
        }
    }
    Ok(())
}

fn write_data_rows(
    f: &mut fmt::Formatter<'_>,
    rows: &[String],
    columns: &[String],
    values: &HashMap<(String, String), String>,
) -> fmt::Result {
    let number_of_rows = rows.len();
    for (i, row) in rows.iter().enumerate() {
        write!(f, "{}", escape_commas(row))?;
        for col in columns {
            write!(f, ",")?;
            if let Some(value) = values.get(&(row.to_string(), col.to_string())) {
                write!(f, "{}", escape_commas(value))?;
            }
        }

        // Only write newline if this isn't the last row
        if i < number_of_rows - 1 {
            writeln!(f)?;
        }
    }

    Ok(())
}

impl fmt::Display for Dataview {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_header_row(f, &self.row_header, &self.column_order)?;
        write_headlines(f, &self.headline_order, &self.headlines)?;
        write_data_rows(f, &self.row_order, &self.column_order, &self.values)
    }
}

impl Dataview {
    /// Creates a new DataviewBuilder instance
    ///
    /// This allows users to create a Dataview without explicitly importing DataviewBuilder
    ///
    /// # Example
    ///
    /// ```
    /// use geneos_toolkit::prelude::*;
    /// let dataview = Dataview::builder()
    ///     .set_row_header("ID")
    ///     .add_headline("Total", "42")
    ///     .add_value("1", "Name", "Alice")
    ///     .build();
    /// ```
    pub fn builder() -> DataviewBuilder {
        DataviewBuilder::new()
    }
}

/// A helper struct to build a row of data.
///
/// This allows constructing a row with multiple columns before adding it to the Dataview.
#[derive(Debug, Clone, Default)]
pub struct Row {
    name: String,
    cells: Vec<(String, String)>,
}

impl Row {
    /// Creates a new Row with the given name (row identifier).
    pub fn new(name: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            cells: Vec::new(),
        }
    }

    /// Adds a cell (column and value) to the row, preserving insertion order.
    pub fn add_cell(mut self, column: impl ToString, value: impl ToString) -> Self {
        self.cells.push((column.to_string(), value.to_string()));
        self
    }
}

/// A Builder for the `Dataview` struct.
#[derive(Debug, Default, Clone)]
pub struct DataviewBuilder {
    row_header: Option<String>,
    headlines: Option<HashMap<String, String>>,
    values: Option<HashMap<(String, String), String>>,
    headline_order: Vec<String>, // for the purpose of ordering the headlines
    column_order: Vec<String>,   // for the purpose of ordering the columns
    row_order: Vec<String>,      // for the purpose of ordering the rows
}

impl DataviewBuilder {
    /// Creates a new, empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the mandatory row header label.
    pub fn set_row_header(mut self, row_header: &str) -> Self {
        self.row_header = Some(row_header.to_string());
        self
    }

    /// Adds or replaces a headline value. Order is preserved by first insert.
    pub fn add_headline<T: ToString>(mut self, key: &str, value: T) -> Self {
        let mut headlines: HashMap<String, String> = self.headlines.unwrap_or_default();

        let key_string = key.to_string();
        if !self.headline_order.contains(&key_string) {
            self.headline_order.push(key_string.clone());
        }

        headlines.insert(key_string, value.to_string());
        self.headlines = Some(headlines);
        self
    }

    /// Adds a single cell value at `row`/`column`, recording insertion order.
    pub fn add_value<T: ToString>(mut self, row: &str, column: &str, value: T) -> Self {
        let mut values: HashMap<(String, String), String> = self.values.unwrap_or_default();

        // Track columns in order of insertion (if new)
        let column_string = column.to_string();
        if !self.column_order.contains(&column_string) {
            self.column_order.push(column_string.clone());
        }

        // Track rows in order of insertion (if new)
        let row_string = row.to_string();
        if !self.row_order.contains(&row_string) {
            self.row_order.push(row_string.clone());
        }

        values.insert((row_string, column_string), value.to_string());
        self.values = Some(values);
        self
    }

    /// Adds a complete row to the Dataview.
    ///
    /// This is a convenience method to add multiple values for the same row at once.
    ///
    /// # Example
    /// ```
    /// use geneos_toolkit::prelude::*;
    ///
    /// let row = Row::new("process1")
    ///     .add_cell("Status", "Running")
    ///     .add_cell("CPU", "2.5%");
    ///
    /// let dataview = Dataview::builder()
    ///     .set_row_header("Process")
    ///     .add_row(row)
    ///     .build();
    /// ```
    pub fn add_row(mut self, row: Row) -> Self {
        for (col, val) in row.cells {
            self = self.add_value(&row.name, &col, &val);
        }
        self
    }

    /// Sorts rows in ascending order by row name. Opt-in; default is insertion order.
    /// Sorts rows in ascending order by row name. Opt-in; default is insertion order.
    pub fn sort_rows(mut self) -> Self {
        self.row_order.sort();
        self
    }

    /// Sorts rows using a key selector. Opt-in; default is insertion order.
    pub fn sort_rows_by<K, F>(mut self, mut f: F) -> Self
    where
        K: Ord,
        F: FnMut(&str) -> K,
    {
        self.row_order.sort_by_key(|row| f(row));
        self
    }

    /// Sorts rows using a custom comparator. Opt-in; default is insertion order.
    pub fn sort_rows_with<F>(mut self, mut cmp: F) -> Self
    where
        F: FnMut(&str, &str) -> std::cmp::Ordering,
    {
        self.row_order.sort_by(|a, b| cmp(a, b));
        self
    }

    /// Builds the `Dataview`, consuming the builder.
    ///
    /// The `row_header` must be set before the build or a panic will occur.
    /// There must be at least one value.
    /// Headlines are optional.
    ///
    /// The order of the columns and rows is determined by the order in which they are added through
    /// values using the `add_value` method.
    ///
    /// The order of headlines is determined by the order in which they are added through the
    /// `add_headline` method.
    ///
    /// Example:
    /// ```rust
    /// use geneos_toolkit::prelude::*;
    ///
    /// let view: Dataview = Dataview::builder()
    ///     .set_row_header("Name")
    ///     .add_headline("AverageAge", "30")
    ///     .add_value("Anna", "Age", "30")
    ///     .add_value("Bertil", "Age", "20")
    ///     .add_value("Caesar", "Age", "40")
    ///     .build()
    ///     .unwrap();
    ///
    /// ```
    pub fn build(self) -> Result<Dataview, DataviewError> {
        let row_header = self.row_header.ok_or(DataviewError::MissingRowHeader)?;

        let values = self.values.ok_or(DataviewError::MissingValue)?;

        Ok(Dataview {
            row_header,
            headlines: self.headlines.unwrap_or_default(),
            headline_order: self.headline_order,
            values,
            column_order: self.column_order,
            row_order: self.row_order,
        })
    }
}

/// Prints the result of a Dataview operation and exits the program.
///
/// # Arguments
/// - `dataview`: The `Result` of a Dataview operation, holding either a `Dataview` or a `DataviewError`.
///
/// # Returns
/// - Exits the program with a status code of 0 if successful, or 1 if an error occurred.
///
/// # Example
/// ```rust
/// use geneos_toolkit::prelude::*;
///
/// let dataview = Dataview::builder()
///    .set_row_header("ID")
///    .add_headline("Total", "42")
///    .add_value("1", "Name", "Alice")
///    .build();
///
/// print_result_and_exit(dataview)
/// ```
/// Prints the dataview on success or an error on failure, then exits the process.
pub fn print_result_and_exit(dataview: Result<Dataview, DataviewError>) -> ! {
    match dataview {
        Ok(v) => {
            println!("{v}");
            std::process::exit(0)
        }
        Err(e) => {
            eprintln!("ERROR: {e}");
            std::process::exit(1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    /// Helper function to create a basic dataview for testing
    fn create_basic_dataview() -> Result<Dataview, DataviewError> {
        DataviewBuilder::new()
            .set_row_header("ID")
            .add_headline("AverageAge", "30")
            .add_value("1", "Name", "Alice")
            .add_value("1", "Age", "30")
            .build()
    }

    #[test]
    fn test_dataview_builder_single_row() -> Result<(), DataviewError> {
        let dataview = create_basic_dataview()?;

        // Test row header
        assert_eq!(dataview.row_header(), "ID");

        // Test headline
        assert_eq!(dataview.headline("AverageAge"), Some(&"30".to_string()));

        // Test values
        assert_eq!(dataview.value("1", "Name"), Some(&"Alice".to_string()));
        assert_eq!(dataview.value("1", "Age"), Some(&"30".to_string()));

        // Test structure
        assert_eq!(dataview.row_order().len(), 1);
        assert_eq!(dataview.column_order().len(), 2);
        assert!(dataview.column_order().contains(&"Name".to_string()));
        assert!(dataview.column_order().contains(&"Age".to_string()));

        Ok(())
    }

    #[test]
    fn test_dataview_display_format() -> Result<(), DataviewError> {
        // Test basic display
        let dataview = create_basic_dataview()?;
        assert_eq!(
            dataview.to_string(),
            "\
ID,Name,Age
<!>AverageAge,30
1,Alice,30"
        );

        // Test multiple rows and columns
        let multi_row_dataview = DataviewBuilder::new()
            .set_row_header("id")
            // Ensure that headlines appear in the order in which they were added.
            .add_headline("Baz", "Foo")
            .add_headline("AlertDetails", "this is red alert")
            .add_value("001", "name", "agila")
            .add_value("001", "status", "up")
            .add_value("001", "Value", "97")
            .add_value("002", "name", "lawin")
            .add_value("002", "status", "down")
            .add_value("002", "Value", "85")
            .build()?;

        let expected_output = "\
id,name,status,Value
<!>Baz,Foo
<!>AlertDetails,this is red alert
001,agila,up,97
002,lawin,down,85";

        assert_eq!(multi_row_dataview.to_string(), expected_output);

        Ok(())
    }

    #[test]
    fn test_special_characters_escaping() -> Result<(), DataviewError> {
        // Test comma escaping in row header, columns, values
        let dataview = DataviewBuilder::new()
            .set_row_header("queue,id")
            .add_value("queue3", "number,code", "7,331")
            .add_value("queue3", "count", "45,000")
            .add_value("queue3", "ratio", "0.16")
            .add_value("queue3", "status", "online")
            .build()?;

        let expected_output = "\
queue\\,id,number\\,code,count,ratio,status
queue3,7\\,331,45\\,000,0.16,online";

        assert_eq!(dataview.to_string(), expected_output);

        // Test other special characters
        let dataview_special = DataviewBuilder::new()
            .set_row_header("special")
            .add_headline("special,headline", "headline value with, comma")
            .add_value("special_case", "state", "testing: \"quotes\" & <symbols>")
            .add_value("special_case", "data", "multi-line\ntext")
            .build()?;

        let output = dataview_special.to_string();
        assert!(output.contains("special"));
        assert!(output.contains("<!>special\\,headline,headline value with\\, comma"));
        assert!(output.contains("testing: \"quotes\" & <symbols>"));
        assert!(output.contains("multi-line\ntext"));

        Ok(())
    }

    #[test]
    fn test_empty_and_missing_values() -> Result<(), DataviewError> {
        // Test with some missing values
        let dataview = DataviewBuilder::new()
            .set_row_header("item")
            .add_value("item1", "col1", "value1")
            .add_value("item1", "col2", "value2")
            .add_value("item2", "col1", "value3")
            // Deliberately missing item2/col2
            .add_value("item3", "col3", "value4") // New column not in other rows
            .build()?;

        let output = dataview.to_string();

        // Verify output format has empty cells where expected
        assert!(output.contains("item1,value1,value2,"));
        assert!(output.contains("item2,value3,,"));
        assert!(output.contains("item3,,,value4"));

        // Test accessing missing values
        assert_eq!(dataview.value("item2", "col2"), None);
        assert_eq!(dataview.value("nonexistent", "col1"), None);

        Ok(())
    }

    #[test]
    fn test_dataview_complex() -> Result<(), DataviewError> {
        // This test creates a more realistic Dataview with many rows, columns and headlines
        let dataview = DataviewBuilder::new()
            .set_row_header("cpu")
            // Add multiple headlines
            .add_headline("numOnlineCpus", "4")
            .add_headline("loadAverage1Min", "0.32")
            .add_headline("loadAverage5Min", "0.45")
            .add_headline("loadAverage15Min", "0.38")
            .add_headline("HyperThreadingStatus", "ENABLED")
            // CPU average row
            .add_value("Average_cpu", "percentUtilisation", "3.75 %")
            .add_value("Average_cpu", "percentUserTime", "2.15 %")
            .add_value("Average_cpu", "percentKernelTime", "1.25 %")
            .add_value("Average_cpu", "percentIdle", "96.25 %")
            // CPU 0 with values in all columns
            .add_value("cpu_0", "type", "GenuineIntel Intel(R)")
            .add_value("cpu_0", "state", "on-line")
            .add_value("cpu_0", "clockSpeed", "2500.00 MHz")
            .add_value("cpu_0", "percentUtilisation", "3.25 %")
            .add_value("cpu_0", "percentUserTime", "1.95 %")
            .add_value("cpu_0", "percentKernelTime", "1.30 %")
            .add_value("cpu_0", "percentIdle", "96.75 %")
            // CPU 1 with same structure
            .add_value("cpu_1", "type", "GenuineIntel Intel(R)")
            .add_value("cpu_1", "state", "on-line")
            .add_value("cpu_1", "clockSpeed", "2500.00 MHz")
            .add_value("cpu_1", "percentUtilisation", "4.25 %")
            .add_value("cpu_1", "percentUserTime", "2.35 %")
            .add_value("cpu_1", "percentKernelTime", "1.20 %")
            .add_value("cpu_1", "percentIdle", "95.75 %")
            // cpu_2 with a comma in one value (needs escaping) and some missing values
            .add_value("cpu_2", "type", "GenuineIntel, Intel(R)")
            .add_value("cpu_2", "state", "on-line")
            .add_value("cpu_2", "clockSpeed", "2,500.00 MHz")
            // Add another logical CPU
            .add_value("cpu_0_logical#1", "type", "logical")
            .add_value("cpu_0_logical#1", "state", "on-line")
            .add_value("cpu_0_logical#1", "clockSpeed", "2500.00 MHz")
            .add_value("cpu_0_logical#1", "percentUtilisation", "2.54 %")
            .build()?;

        // Get the output
        let output = dataview.to_string();

        // Check structure
        assert_eq!(dataview.row_order().len(), 5); // 5 rows
        assert_eq!(dataview.row_order()[0], "Average_cpu".to_string());
        assert_eq!(dataview.row_order()[1], "cpu_0".to_string());
        assert_eq!(dataview.row_order()[2], "cpu_1".to_string());
        assert_eq!(dataview.row_order()[3], "cpu_2".to_string());
        assert_eq!(dataview.row_order()[4], "cpu_0_logical#1".to_string());

        assert_eq!(dataview.headlines.len(), 5); // 5 headlines

        // Assert column ordering is preserved
        let expected_columns = [
            "percentUtilisation",
            "percentUserTime",
            "percentKernelTime",
            "percentIdle",
            "type",
            "state",
            "clockSpeed",
        ];
        for (idx, col) in expected_columns.iter().enumerate() {
            if idx < dataview.column_order().len() {
                assert!(dataview.column_order().contains(&col.to_string()));
            }
        }

        // Basic format checks
        assert!(output.starts_with("cpu,"));
        assert!(output.contains("<!>numOnlineCpus,4\n"));
        assert!(output.contains("<!>loadAverage1Min,0.32\n"));
        assert!(output.contains("<!>HyperThreadingStatus,ENABLED\n"));

        // Check comma escaping
        assert!(output.contains("GenuineIntel\\, Intel(R)"));
        assert!(output.contains("2\\,500.00 MHz"));

        Ok(())
    }

    #[test]
    fn test_error_conditions() -> Result<(), ()> {
        // Test missing row header
        let result = DataviewBuilder::new()
            .add_value("row1", "col1", "value1")
            .build();

        assert!(matches!(result, Err(DataviewError::MissingRowHeader)));

        // Test missing values
        let result = DataviewBuilder::new().set_row_header("header").build();

        assert!(matches!(result, Err(DataviewError::MissingValue)));

        // Ensure headlines alone are not enough
        let result = DataviewBuilder::new()
            .set_row_header("header")
            .add_headline("headline1", "value1")
            .build();

        assert!(matches!(result, Err(DataviewError::MissingValue)));

        Ok(())
    }

    #[test]
    fn test_row_builder() -> Result<(), DataviewError> {
        let row1 = Row::new("process1")
            .add_cell("Status", "Running")
            .add_cell("CPU", "2.5%");

        let row2 = Row::new("process2")
            .add_cell("Status", "Stopped")
            .add_cell("CPU", "0.0%");

        let dataview = Dataview::builder()
            .set_row_header("Process")
            .add_row(row1)
            .add_row(row2)
            .build()?;

        let output = dataview.to_string();

        assert!(output.contains("Process,Status,CPU"));
        assert!(output.contains("process1,Running,2.5%"));
        assert!(output.contains("process2,Stopped,0.0%"));

        Ok(())
    }

    #[test]
    fn test_row_sorting_methods() -> Result<(), DataviewError> {
        // Default: insertion order preserved
        let default = Dataview::builder()
            .set_row_header("id")
            .add_value("b", "col", "1")
            .add_value("a", "col", "1")
            .add_value("c", "col", "1")
            .build()?;
        assert_eq!(default.row_order(), &["b", "a", "c"]);

        // sort_rows: ascending by row name
        let sorted = Dataview::builder()
            .set_row_header("id")
            .add_value("b", "col", "1")
            .add_value("a", "col", "1")
            .add_value("c", "col", "1")
            .sort_rows()
            .build()?;
        assert_eq!(sorted.row_order(), &["a", "b", "c"]);

        // sort_rows_by: custom key (length)
        let by_len = Dataview::builder()
            .set_row_header("id")
            .add_row(Row::new("long").add_cell("v", "1"))
            .add_row(Row::new("mid").add_cell("v", "1"))
            .add_row(Row::new("s").add_cell("v", "1"))
            .sort_rows_by(|name| name.len())
            .build()?;
        assert_eq!(by_len.row_order(), &["s", "mid", "long"]);

        // sort_rows_with: custom comparator (reverse lexicographic)
        let reversed = Dataview::builder()
            .set_row_header("id")
            .add_row(Row::new("alpha").add_cell("v", "1"))
            .add_row(Row::new("beta").add_cell("v", "1"))
            .add_row(Row::new("gamma").add_cell("v", "1"))
            .sort_rows_with(|a, b| b.cmp(a))
            .build()?;
        assert_eq!(reversed.row_order(), &["gamma", "beta", "alpha"]);

        Ok(())
    }
}
