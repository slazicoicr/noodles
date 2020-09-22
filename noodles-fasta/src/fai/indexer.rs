//! Index creation for a FASTA file as specified at http://www.htslib.org/doc/faidx.html

use std::{error::Error, fmt, io, io::BufRead};

use memchr::memchr;

use crate::reader::{DEFINITION_PREFIX, NEWLINE};
use crate::record::definition::{Definition, ParseError};

use super::Record;

/// Creates an index from a FASTA file.
///
/// The position of the stream is expected to be at the beginning of the file.
///
/// # Examples
/// ```
/// use noodles_fasta::fai::{index, Record};
/// let data = b"\
/// >one
/// ATGCAT
/// GCATGC
/// ATG
/// >two another chromosome
/// ATGC
/// GCAT";
///
/// let indx = index(&data[..]).expect("Failed to read or invalid format");
/// let expected = vec![
///     Record::new("one".to_string(), 15, 5, 6, 7),
///     Record::new("two".to_string(), 8, 47, 4, 5)
/// ];
///
/// assert_eq!(indx, expected);
/// ```
pub fn index<R: BufRead>(buf: R) -> Result<Vec<Record>, IndexError> {
    let mut indexer = Indexer::new(buf);
    let mut result = Vec::new();

    loop {
        match indexer.index_record()? {
            Some(i) => result.push(i),
            None => break,
        };
    }

    return Ok(result);
}

/// An index builder.
pub struct Indexer<R> {
    inner: R,
    offset: u64,
}

impl<R> Indexer<R>
where
    R: BufRead,
{
    /// Creates a FASTA index builder.
    pub fn new(inner: R) -> Self {
        Self { inner, offset: 0 }
    }

    /// Reads a single sequence line.
    ///
    /// Trailing whitespaces are not modified.
    ///
    /// Returns the number of bytes read from the stream. If the position of the stream is at
    /// a definition or EOF has been reached, the number of bytes read is 0.
    fn read_sequence_line(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        let mut bytes_read = 0;
        let mut end_line = false;

        loop {
            let reader_buf = self.inner.fill_buf()?;

            if end_line || reader_buf.is_empty() || reader_buf[0] == DEFINITION_PREFIX {
                break;
            }

            let len = match memchr(NEWLINE, reader_buf) {
                Some(i) => {
                    // Do not consume newline.
                    buf.extend(&reader_buf[..=i]);
                    end_line = true;
                    i + 1
                }
                None => {
                    buf.extend(reader_buf);
                    reader_buf.len()
                }
            };

            self.inner.consume(len);

            bytes_read += len;
        }

        Ok(bytes_read)
    }

    /// Creates a single index [`Record`](struct.Record.html).
    ///
    /// Returns `None` if EOF has been reached. Will return an error if:
    /// * Stream is not at the start of a definition
    /// * No sequence after a definition
    /// * Sequence lines are not the same length (excluding the last line)
    pub fn index_record(&mut self) -> Result<Option<Record>, IndexError> {
        let mut def_str = String::new();
        let mut line = Vec::new();
        let mut length = 0u64;

        let def_len = self.inner.read_line(&mut def_str)?;
        if def_len == 0 {
            return Ok(None);
        }

        self.offset += def_len as u64;
        let index_offset = self.offset;
        let def: Definition = def_str.trim_end().parse()?;

        // The first sequence line determines how long each line should be.
        let mut seq_len = self.read_sequence_line(&mut line)? as u64;
        let line_width = seq_len;
        let line_bases = len_with_right_trim(&line) as u64;

        loop {
            let prev_line_width = seq_len;
            let prev_line_bases = len_with_right_trim(&line) as u64;
            length += prev_line_bases;
            self.offset += prev_line_width;

            line.clear();
            seq_len = self.read_sequence_line(&mut line)? as u64;
            if seq_len == 0 {
                break;
            // If there are more lines, check the previous line has equal length to first.
            } else if prev_line_width != line_width || prev_line_bases != line_bases {
                return Err(IndexError::InvalidLineLength(self.offset));
            }
        }

        if length == 0 {
            return Err(IndexError::EmptyDefinition(self.offset));
        }

        let record = Record::new(
            def.reference_sequence_name().to_string(),
            length,
            index_offset,
            line_bases,
            line_width,
        );

        Ok(Some(record))
    }
}

fn len_with_right_trim(vec: &[u8]) -> usize {
    match vec.iter().rposition(|x| !x.is_ascii_whitespace()) {
        Some(i) => i + 1,
        None => 0,
    }
}

#[derive(Debug)]
pub enum IndexError {
    EmptyDefinition(u64),
    InvalidDefinition(ParseError),
    InvalidLineLength(u64),
    IoError(io::Error),
}

impl Error for IndexError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::EmptyDefinition(_) => None,
            Self::InvalidDefinition(e) => Some(e),
            Self::InvalidLineLength(_) => None,
            Self::IoError(e) => Some(e),
        }
    }
}

impl fmt::Display for IndexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyDefinition(offset) => write!(f, "Empty definition at offset {}", offset),
            Self::InvalidDefinition(e) => e.fmt(f),
            Self::InvalidLineLength(offset) => {
                write!(f, "Different line lengths at offset {}", offset)
            }
            Self::IoError(e) => e.fmt(f),
        }
    }
}

impl From<io::Error> for IndexError {
    fn from(error: io::Error) -> Self {
        Self::IoError(error)
    }
}

impl From<ParseError> for IndexError {
    fn from(error: ParseError) -> Self {
        Self::InvalidDefinition(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_sequence_line() {
        let data = b"ACGT\nNNNN\n";
        let mut builder = Indexer::new(&data[..]);

        let mut buf = Vec::new();
        builder
            .read_sequence_line(&mut buf)
            .expect("Failed to read sequence");

        assert_eq!(buf, b"ACGT\n");
    }

    #[test]
    fn test_index_record() {
        let data = b">sq0\nACGT\n>sq1\nNNNN\nNNNN\nNN";
        let mut builder = Indexer::new(&data[..]);

        let first_index = builder.index_record().expect("Could not parse fasta file");
        assert_eq!(
            first_index,
            Some(Record::new("sq0".to_string(), 4, 5, 4, 5))
        );
        let second_index = builder.index_record().expect("Could not parse fasta file");
        assert_eq!(
            second_index,
            Some(Record::new("sq1".to_string(), 10, 15, 4, 5))
        );
    }

    #[test]
    fn test_len_with_right_trim() {
        assert_eq!(len_with_right_trim(b"ATGC\n"), 4);
        assert_eq!(len_with_right_trim(b"ATGC\r\n"), 4);
    }
}
