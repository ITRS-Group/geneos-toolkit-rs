use std::collections::HashMap;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum DataviewError {
    MissingRowHeader,
    MissingValue,
    EmptyName(String),
}

impl fmt::Display for DataviewError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataviewError::MissingRowHeader => write!(f, "The Dataview must have a row header"),
            DataviewError::MissingValue => write!(f, "The Dataview must have at least one value"),
            DataviewError::EmptyName(field) => write!(f, "Empty {field} name is not allowed"),
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

/// Strips Unicode control characters (categories Cc and Cf) except ASCII
/// whitespace (tab, newline, carriage return, space). Newlines and carriage
/// returns are subsequently escaped by `escape_nasty_chars`.
fn strip_unicode_controls(s: &str) -> String {
    s.chars()
        .filter(|&c| {
            if c == '\t' || c == '\n' || c == '\r' || c == ' ' {
                return true;
            }
            !c.is_control() && !is_unicode_format_char(c)
        })
        .collect()
}

/// Returns `true` for Unicode Cf (format) characters — RTL override, zero-width
/// space, BOM, etc. Rust's `char::is_control()` covers Cc; this covers Cf.
fn is_unicode_format_char(c: char) -> bool {
    matches!(c as u32,
        0x00AD              // SOFT HYPHEN
        | 0x0600..=0x0605   // Arabic format chars
        | 0x061C            // ARABIC LETTER MARK
        | 0x06DD            // ARABIC END OF AYAH
        | 0x070F            // SYRIAC ABBREVIATION MARK
        | 0x08E2            // ARABIC DISPUTED END OF AYAH
        | 0x180E            // MONGOLIAN VOWEL SEPARATOR
        | 0x200B..=0x200F   // Zero-width space, joiners, LTR/RTL marks
        | 0x202A..=0x202E   // Directional formatting
        | 0x2060..=0x2064   // Word joiner, invisible operators
        | 0x2066..=0x206F   // Directional isolates, deprecated chars
        | 0xFEFF            // BOM / ZWNBSP
        | 0xFFF9..=0xFFFB   // Interlinear annotations
        | 0x110BD           // KAITHI NUMBER SIGN
        | 0x110CD           // KAITHI NUMBER SIGN ABOVE
        | 0x13430..=0x13438 // Egyptian hieroglyph format
        | 0x1BCA0..=0x1BCA3 // Shorthand format controls
        | 0x1D173..=0x1D17A // Musical symbol format
        | 0xE0001           // LANGUAGE TAG
        | 0xE0020..=0xE007F // TAG characters
    )
}

trait GeneosEscaping {
    fn escape_nasty_chars(&self) -> String;
}

impl GeneosEscaping for str {
    fn escape_nasty_chars(&self) -> String {
        let mut output = String::with_capacity(self.len());

        // C1: Escape <!> at string start to prevent headline injection
        let s = if let Some(rest) = self.strip_prefix("<!>") {
            output.push_str("\\<!>");
            rest
        } else {
            self
        };

        for c in s.chars() {
            match c {
                '\\' => output.push_str("\\\\"),
                ',' => output.push_str("\\,"),
                '\n' => output.push_str("\\n"),
                '\r' => output.push_str("\\r"),
                '\0' => output.push_str("\\0"),
                c => output.push(c),
            }
        }
        output
    }
}

fn write_header_row(
    f: &mut fmt::Formatter<'_>,
    row_header: &str,
    columns: &[String],
) -> fmt::Result {
    write!(f, "{}", row_header.escape_nasty_chars())?;
    for col in columns {
        write!(f, ",{}", col.escape_nasty_chars())?;
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
            writeln!(
                f,
                "<!>{},{}",
                name.escape_nasty_chars(),
                value.escape_nasty_chars()
            )?;
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
        write!(f, "{}", row.escape_nasty_chars())?;
        for col in columns {
            write!(f, ",")?;
            if let Some(value) = values.get(&(row.to_string(), col.to_string())) {
                write!(f, "{}", value.escape_nasty_chars())?;
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
#[derive(Debug, Clone)]
pub struct DataviewBuilder {
    row_header: Option<String>,
    headlines: Option<HashMap<String, String>>,
    values: Option<HashMap<(String, String), String>>,
    headline_order: Vec<String>, // for the purpose of ordering the headlines
    column_order: Vec<String>,   // for the purpose of ordering the columns
    row_order: Vec<String>,      // for the purpose of ordering the rows
    strip_unicode: bool,
}

impl Default for DataviewBuilder {
    fn default() -> Self {
        Self {
            row_header: None,
            headlines: None,
            values: None,
            headline_order: Vec::new(),
            column_order: Vec::new(),
            row_order: Vec::new(),
            strip_unicode: true,
        }
    }
}

impl DataviewBuilder {
    /// Creates a new, empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Controls whether Unicode control characters (categories Cc and Cf,
    /// excluding ASCII whitespace) are stripped from all input strings.
    /// Enabled by default. Set to `false` to preserve raw Unicode control characters.
    pub fn strip_unicode_controls(mut self, strip: bool) -> Self {
        self.strip_unicode = strip;
        self
    }

    /// Sanitize a string according to builder settings.
    fn sanitize(&self, s: &str) -> String {
        if self.strip_unicode {
            strip_unicode_controls(s)
        } else {
            s.to_string()
        }
    }

    /// Sets the mandatory row header label.
    pub fn set_row_header(mut self, row_header: &str) -> Self {
        self.row_header = Some(self.sanitize(row_header));
        self
    }

    /// Adds or replaces a headline value. Order is preserved by first insert.
    pub fn add_headline<T: ToString>(mut self, key: &str, value: T) -> Self {
        let key_string = self.sanitize(key);
        let value_string = self.sanitize(&value.to_string());

        let mut headlines: HashMap<String, String> = self.headlines.unwrap_or_default();

        if !self.headline_order.contains(&key_string) {
            self.headline_order.push(key_string.clone());
        }

        headlines.insert(key_string, value_string);
        self.headlines = Some(headlines);
        self
    }

    /// Adds a single cell value at `row`/`column`, recording insertion order.
    pub fn add_value<T: ToString>(mut self, row: &str, column: &str, value: T) -> Self {
        let column_string = self.sanitize(column);
        let row_string = self.sanitize(row);
        let value_string = self.sanitize(&value.to_string());

        let mut values: HashMap<(String, String), String> = self.values.unwrap_or_default();

        // Track columns in order of insertion (if new)
        if !self.column_order.contains(&column_string) {
            self.column_order.push(column_string.clone());
        }

        // Track rows in order of insertion (if new)
        if !self.row_order.contains(&row_string) {
            self.row_order.push(row_string.clone());
        }

        values.insert((row_string, column_string), value_string);
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

        if row_header.is_empty() {
            return Err(DataviewError::EmptyName("row header".into()));
        }

        let values = self.values.ok_or(DataviewError::MissingValue)?;

        for row in &self.row_order {
            if row.is_empty() {
                return Err(DataviewError::EmptyName("row".into()));
            }
        }

        for col in &self.column_order {
            if col.is_empty() {
                return Err(DataviewError::EmptyName("column".into()));
            }
        }

        if let Some(ref headlines) = self.headlines {
            for key in headlines.keys() {
                if key.is_empty() {
                    return Err(DataviewError::EmptyName("headline".into()));
                }
            }
        }

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
        assert!(output.contains("multi-line\\ntext"));

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
    fn test_duplicate_headline_overwrites_value_preserves_order() -> Result<(), DataviewError> {
        let dataview = DataviewBuilder::new()
            .set_row_header("id")
            .add_headline("Status", "initial")
            .add_headline("Count", "10")
            .add_headline("Status", "updated")
            .add_value("r1", "col", "val")
            .build()?;

        // Value should be overwritten
        assert_eq!(dataview.headline("Status"), Some(&"updated".to_string()));
        assert_eq!(dataview.headline("Count"), Some(&"10".to_string()));

        // Order should reflect first insertion: Status before Count
        assert_eq!(dataview.headline_order(), &["Status", "Count"]);

        // Display should use updated value in original position
        let output = dataview.to_string();
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines[1], "<!>Status,updated");
        assert_eq!(lines[2], "<!>Count,10");

        Ok(())
    }

    #[test]
    fn test_duplicate_cell_overwrites_value_preserves_order() -> Result<(), DataviewError> {
        let dataview = DataviewBuilder::new()
            .set_row_header("id")
            .add_value("row1", "colA", "first")
            .add_value("row1", "colB", "other")
            .add_value("row2", "colA", "x")
            .add_value("row1", "colA", "second")
            .build()?;

        // Value should be overwritten
        assert_eq!(dataview.value("row1", "colA"), Some(&"second".to_string()));

        // Row and column order should reflect first insertion only (no duplicates)
        assert_eq!(dataview.row_order(), &["row1", "row2"]);
        assert_eq!(dataview.column_order(), &["colA", "colB"]);

        // Display should use the overwritten value
        let output = dataview.to_string();
        assert!(output.contains("row1,second,other"));

        Ok(())
    }

    #[test]
    fn test_backslash_escaping() -> Result<(), DataviewError> {
        let dataview = DataviewBuilder::new()
            .set_row_header("path\\id")
            .add_headline("dir", "C:\\Users\\test")
            .add_value("row\\1", "col\\a", "val\\ue")
            .build()?;

        let output = dataview.to_string();
        let lines: Vec<&str> = output.lines().collect();

        assert_eq!(lines[0], "path\\\\id,col\\\\a");
        assert_eq!(lines[1], "<!>dir,C:\\\\Users\\\\test");
        assert_eq!(lines[2], "row\\\\1,val\\\\ue");

        Ok(())
    }

    #[test]
    fn test_accessor_methods_nonexistent_keys() -> Result<(), DataviewError> {
        let dataview = DataviewBuilder::new()
            .set_row_header("id")
            .add_headline("exists", "yes")
            .add_value("row1", "col1", "val1")
            .build()?;

        assert_eq!(dataview.headline("nonexistent"), None);
        assert_eq!(dataview.value("row1", "nonexistent"), None);
        assert_eq!(dataview.value("nonexistent", "col1"), None);
        assert_eq!(dataview.value("nonexistent", "nonexistent"), None);

        Ok(())
    }

    #[test]
    fn test_dataview_no_headlines() -> Result<(), DataviewError> {
        let dataview = DataviewBuilder::new()
            .set_row_header("item")
            .add_value("a", "x", "1")
            .add_value("b", "x", "2")
            .build()?;

        let output = dataview.to_string();
        assert!(!output.contains("<!>"));
        assert_eq!(output, "item,x\na,1\nb,2");

        Ok(())
    }

    #[test]
    fn test_golden_snapshot_representative_dataview() -> Result<(), DataviewError> {
        let dataview = DataviewBuilder::new()
            .set_row_header("service")
            .add_headline("environment", "production")
            .add_headline("region", "eu-west-1")
            .add_value("api-gateway", "status", "running")
            .add_value("api-gateway", "latency_ms", "12")
            .add_value("api-gateway", "errors", "0")
            .add_value("db-primary", "status", "running")
            .add_value("db-primary", "latency_ms", "3")
            .add_value("db-primary", "errors", "0")
            .add_value("cache", "status", "degraded")
            .add_value("cache", "latency_ms", "45")
            .add_value("cache", "errors", "7")
            .build()?;

        let expected = "\
service,status,latency_ms,errors
<!>environment,production
<!>region,eu-west-1
api-gateway,running,12,0
db-primary,running,3,0
cache,degraded,45,7";

        assert_eq!(dataview.to_string(), expected);

        Ok(())
    }

    // === C1: <!> headline injection ===

    #[test]
    fn test_escape_headline_prefix_in_row_name() -> Result<(), DataviewError> {
        // A row name starting with <!> must not render as a headline
        let dataview = Dataview::builder()
            .set_row_header("id")
            .add_value("<!>AlertSeverity,OK", "status", "injected")
            .build()?;

        let output = dataview.to_string();
        // The data row must NOT start with raw <!>
        let data_lines: Vec<&str> = output.lines().filter(|l| !l.starts_with("<!>")).collect();
        assert!(data_lines.len() >= 2, "Should have header + data row");
        let data_row = data_lines[1];
        assert!(
            !data_row.starts_with("<!>"),
            "Row name must not produce a fake headline: {data_row}"
        );
        // The escaped form should appear
        assert!(data_row.contains("\\<!>"));

        Ok(())
    }

    #[test]
    fn test_escape_headline_prefix_in_value() -> Result<(), DataviewError> {
        // A value starting with <!> should be escaped
        let dataview = Dataview::builder()
            .set_row_header("id")
            .add_value("row1", "col", "<!>Fake,headline")
            .build()?;

        let output = dataview.to_string();
        // Value should contain escaped form
        assert!(output.contains("\\<!>Fake"));

        Ok(())
    }

    #[test]
    fn test_escape_headline_prefix_in_row_header() -> Result<(), DataviewError> {
        // Row header starting with <!> must be escaped
        let dataview = Dataview::builder()
            .set_row_header("<!>header")
            .add_value("row1", "col", "val")
            .build()?;

        let output = dataview.to_string();
        let first_line = output.lines().next().unwrap();
        assert!(
            first_line.starts_with("\\<!>header"),
            "Row header must escape <!>: {first_line}"
        );

        Ok(())
    }

    #[test]
    fn test_headline_prefix_mid_string_not_escaped() {
        // <!> only matters at string start — mid-string is harmless
        let escaped = "some<!>text".escape_nasty_chars();
        assert_eq!(escaped, "some<!>text");
    }

    #[test]
    fn test_real_headlines_unaffected() -> Result<(), DataviewError> {
        // Legitimate headlines must still render with <!> prefix
        let dataview = Dataview::builder()
            .set_row_header("id")
            .add_headline("Status", "OK")
            .add_value("r1", "c1", "v1")
            .build()?;

        let output = dataview.to_string();
        assert!(output.contains("<!>Status,OK"));

        Ok(())
    }

    // === H1: null byte passthrough ===

    #[test]
    fn test_escape_null_byte() {
        let escaped = "before\0after".escape_nasty_chars();
        assert_eq!(escaped, "before\\0after");
        assert!(!escaped.contains('\0'));
    }

    #[test]
    fn test_null_byte_in_value() -> Result<(), DataviewError> {
        // Null bytes are stripped by unicode sanitizer (defense in depth:
        // escape_nasty_chars would also escape them if they got through)
        let dataview = Dataview::builder()
            .set_row_header("id")
            .add_value("row1", "col", "legitimate\0<!>INJECTED")
            .build()?;

        let output = dataview.to_string();
        assert!(
            !output.contains('\0'),
            "Null bytes must not appear in output"
        );
        // \0 is stripped, so "legitimate" and "<!>INJECTED" are concatenated
        assert!(output.contains("legitimate<!>INJECTED"));

        Ok(())
    }

    #[test]
    fn test_null_byte_in_row_name() -> Result<(), DataviewError> {
        let dataview = Dataview::builder()
            .set_row_header("id")
            .add_value("row\01", "col", "val")
            .build()?;

        let output = dataview.to_string();
        assert!(!output.contains('\0'));

        Ok(())
    }

    // === M3: unicode control character stripping ===

    #[test]
    fn test_strip_rtl_override() -> Result<(), DataviewError> {
        // U+202E (RTL override) can make "KO" display as "OK" — must be stripped
        let dataview = Dataview::builder()
            .set_row_header("id")
            .add_value("row1", "status", "\u{202E}KO")
            .build()?;

        let output = dataview.to_string();
        assert!(
            !output.contains('\u{202E}'),
            "RTL override must be stripped"
        );
        assert!(output.contains("KO"));

        Ok(())
    }

    #[test]
    fn test_strip_zero_width_space() -> Result<(), DataviewError> {
        // U+200B (zero-width space) breaks rule matching
        let dataview = Dataview::builder()
            .set_row_header("id")
            .add_value("row1", "col", "OK\u{200B}status")
            .build()?;

        let output = dataview.to_string();
        assert!(!output.contains('\u{200B}'));
        assert!(output.contains("OKstatus"));

        Ok(())
    }

    #[test]
    fn test_strip_bom() -> Result<(), DataviewError> {
        // U+FEFF (BOM) at start can confuse encoding detection
        let dataview = Dataview::builder()
            .set_row_header("id")
            .add_value("row1", "col", "\u{FEFF}value")
            .build()?;

        let output = dataview.to_string();
        assert!(!output.contains('\u{FEFF}'));

        Ok(())
    }

    #[test]
    fn test_preserve_ascii_whitespace() -> Result<(), DataviewError> {
        // Tab and space must survive stripping (they are useful ASCII control/whitespace)
        let dataview = Dataview::builder()
            .set_row_header("id")
            .add_value("row1", "col", "hello\tworld here")
            .build()?;

        let output = dataview.to_string();
        assert!(output.contains("hello\tworld here"));

        Ok(())
    }

    #[test]
    fn test_strip_unicode_controls_opt_out() -> Result<(), DataviewError> {
        // When strip_unicode_controls(false), control chars pass through
        let dataview = Dataview::builder()
            .set_row_header("id")
            .strip_unicode_controls(false)
            .add_value("row1", "col", "\u{202E}KO")
            .build()?;

        let output = dataview.to_string();
        assert!(
            output.contains('\u{202E}'),
            "RTL override should be preserved when stripping is disabled"
        );

        Ok(())
    }

    #[test]
    fn test_strip_unicode_in_headline_key_and_value() -> Result<(), DataviewError> {
        let dataview = Dataview::builder()
            .set_row_header("id")
            .add_headline("stat\u{200B}us", "O\u{202E}K")
            .add_value("r1", "c1", "v1")
            .build()?;

        let output = dataview.to_string();
        assert!(output.contains("<!>status,OK"));

        Ok(())
    }

    #[test]
    fn test_strip_unicode_in_row_and_column_names() -> Result<(), DataviewError> {
        let dataview = Dataview::builder()
            .set_row_header("i\u{FEFF}d")
            .add_value("ro\u{200B}w1", "co\u{202E}l", "val")
            .build()?;

        let output = dataview.to_string();
        let first_line = output.lines().next().unwrap();
        assert_eq!(first_line, "id,col");

        Ok(())
    }

    // === L5: empty row/column name rejection ===

    #[test]
    fn test_reject_empty_row_header() {
        let result = Dataview::builder()
            .set_row_header("")
            .add_value("row1", "col", "val")
            .build();

        assert!(matches!(result, Err(DataviewError::EmptyName(_))));
    }

    #[test]
    fn test_reject_empty_row_name() {
        let result = Dataview::builder()
            .set_row_header("id")
            .add_value("", "col", "val")
            .build();

        assert!(matches!(result, Err(DataviewError::EmptyName(_))));
    }

    #[test]
    fn test_reject_empty_column_name() {
        let result = Dataview::builder()
            .set_row_header("id")
            .add_value("row1", "", "val")
            .build();

        assert!(matches!(result, Err(DataviewError::EmptyName(_))));
    }

    #[test]
    fn test_reject_empty_headline_key() {
        let result = Dataview::builder()
            .set_row_header("id")
            .add_headline("", "val")
            .add_value("row1", "col", "val")
            .build();

        assert!(matches!(result, Err(DataviewError::EmptyName(_))));
    }

    #[test]
    fn test_whitespace_only_name_after_stripping() {
        // A name that is only unicode control chars becomes empty after stripping
        let result = Dataview::builder()
            .set_row_header("id")
            .add_value("\u{200B}\u{FEFF}", "col", "val")
            .build();

        assert!(matches!(result, Err(DataviewError::EmptyName(_))));
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

#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_escape_nasty_chars_no_newlines(s in "\\PC*") {
            let escaped = s.escape_nasty_chars();
            // The escaped string should not contain raw newlines as they break the protocol
            prop_assert!(!escaped.contains('\n'));
            prop_assert!(!escaped.contains('\r'));
        }

        #[test]
        fn test_escape_nasty_chars_no_null_bytes(s in "\\PC*") {
            let escaped = s.escape_nasty_chars();
            prop_assert!(!escaped.contains('\0'), "Null bytes must be escaped");
        }

        #[test]
        fn test_escape_nasty_chars_no_headline_injection(s in "\\PC*") {
            let escaped = s.escape_nasty_chars();
            // If the original started with <!>, the escaped form must not
            if s.starts_with("<!>") {
                prop_assert!(
                    !escaped.starts_with("<!>"),
                    "Escaped string must not start with raw <!>"
                );
            }
        }

        #[test]
        fn test_dataview_structure_integrity_with_newlines(
            row_name in "[a-z]+",
            col_name in "[a-z]+",
            // Explicitly generate strings with newlines and commas
            value in "([a-z]|\n|,|\r)*"
        ) {
            let res = Dataview::builder()
                .set_row_header("row_id")
                .add_value(&row_name, &col_name, &value)
                .build();

            prop_assert!(res.is_ok());
            let view = res.unwrap();
            let output = view.to_string();

            let lines: Vec<&str> = output.lines().collect();

            // Should have exactly 2 lines (header + 1 data row)
            // If value contained \n and wasn't escaped, this will fail
            prop_assert_eq!(lines.len(), 2,
                "Output should have exactly 2 lines, found {}. Value was: {:?}",
                lines.len(), value);

            prop_assert!(lines[1].starts_with(&row_name));
        }

        #[test]
        fn test_dataview_column_count_consistency(
            row_header in "[a-z]+",
            rows in proptest::collection::vec("[a-z]+", 1..10),
            cols in proptest::collection::vec("[a-z]+", 1..10),
            val in "\\PC*"
        ) {
            let mut builder = Dataview::builder().set_row_header(&row_header);

            // Add values for every row/col combination
            for r in &rows {
                for c in &cols {
                    builder = builder.add_value(r, c, &val);
                }
            }

            let view = builder.build().unwrap();
            let output = view.to_string();

            for line in output.lines() {
                // Skip headline lines
                if line.starts_with("<!>") {
                    continue;
                }

                // Let's count occurrences of "," that are NOT preceded by an ODD number of backslashes
                // This is getting complicated to parse with regex/simple checks.
                // A comma is a separator if it is NOT escaped.
                // It is escaped if it is preceded by a backslash that is NOT itself escaped.

                let mut raw_commas = 0;
                let mut chars = line.chars().peekable();
                let mut escaped = false;

                while let Some(c) = chars.next() {
                    if escaped {
                        escaped = false;
                    } else if c == '\\' {
                        escaped = true;
                    } else if c == ',' {
                        raw_commas += 1;
                    }
                }

                // The number of commas should be equal to the number of columns
                // Example: row_header, col1, col2 -> 2 commas for 2 columns
                // Example: row1, val1, val2 -> 2 commas for 2 columns

                let actual_cols = view.column_order().len();

                prop_assert_eq!(raw_commas, actual_cols,
                    "Line has wrong number of columns: {}", line);
            }
        }

        #[test]
        fn test_headline_escaping(
            key in "[a-z]+",
            value in "([a-z]|\n|,|\r)*"
        ) {
            let view = Dataview::builder()
                .set_row_header("id")
                .add_headline(&key, &value)
                .add_value("r", "c", "v")
                .build()
                .unwrap();

            let output = view.to_string();
            // Find the headline line
            let headline_line = output.lines()
                .find(|l| l.starts_with("<!>"))
                .expect("Should have headline");

            // Should be on one line
            prop_assert!(!headline_line.contains('\n'));

            // Should have exactly one unescaped comma separating key and value
            let raw_commas = headline_line.match_indices(',')
                .filter(|(idx, _)| *idx == 0 || headline_line.as_bytes()[idx-1] != b'\\')
                .count();

            prop_assert_eq!(raw_commas, 1, "Headline should have exactly 1 separator comma");
        }
    }
}
