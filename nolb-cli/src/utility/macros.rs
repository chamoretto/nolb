/// Reads file contents into string.
///
/// Example:
/// ```rust
/// use crate::utility::macros::read_file_content;
///
/// let file_content: String = read_file_content!("example.txt");
/// ```
///
/// **Warning**: Macro will panic if cannot open or read file!
pub(crate) macro read_file_content($path:expr) {{
    let mut _string = String::new();
    {
        use std::io::Read;
        let mut _file = std::fs::File::open($path).expect("No test file!");
        <std::fs::File as Read>::read_to_string(&mut _file, &mut _string).expect("Unable to read test file!");
    }
    _string
}}

/// Reads file contents into string.
///
/// Example:
/// ```rust
/// use crate::utility::macros::try_read_file_content;
///
/// let file_content: String = try_read_file_content!("example.txt");
/// ```
///
/// **Warning**: Macro uses question mark operator `?`!
pub(crate) macro try_read_file_content($path:expr) {{
    let mut __string = String::new();
    {
        use std::io::Read;
        let mut __file = std::fs::File::open($path)?;
        <std::fs::File as Read>::read_to_string(&mut __file, &mut __string)?;
    }
    __string
}}
