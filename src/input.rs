use std::io::Read;
use std::path::Path;

/// Build a CSV reader for `path`, or stdin if `path` is `-`.
pub fn reader_for(path: &Path) -> Result<csv::Reader<Box<dyn Read>>, csv::Error> {
    let source: Box<dyn Read> = if path == Path::new("-") {
        Box::new(std::io::stdin())
    } else {
        Box::new(std::fs::File::open(path)?)
    };
    Ok(csv::Reader::from_reader(source))
}
