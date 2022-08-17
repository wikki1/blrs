/**
 * err.rs -- Abstracts away error messages
 */

/**
 * Error Handling Functionality
 * Calling exec() on an error will print 
 * a statement to stderr
 * and exit using std::process::exit
 */
#[derive(Debug)]
pub enum Err {
    UsageSubcommand(String),
    File(std::io::Error),
    ValueParse,
    IllegalUse(String),
    OutOfBounds
}

impl Err {
    pub fn exec(self) {
        let (msg, code) = match self {
            Err::UsageSubcommand(s) => {
                if s == "smooth" {
                    (format!["Usage: blrs {} <value> <time>", s], 1)
                } else {
                    (format!["Usage: blrs {} <value>", s], 1)
                }
            },
            Err::File(e) => (format!["Err: {}", e], 2),
            Err::ValueParse => (String::from("Err: Unable to parse value"), 4),
            Err::IllegalUse(s) => (format!["Err: {}", s], 5),
            Err::OutOfBounds => (String::from("Err: Value out of bounds"), 6)
        };

        eprintln!("{}", msg);
        std::process::exit(code);
    }
}