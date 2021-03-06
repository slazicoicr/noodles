//! SAM header reference sequence and fields.

mod builder;
mod md5_checksum;
mod molecule_topology;
mod tag;

use std::{collections::HashMap, convert::TryFrom, error, fmt, num};

pub use self::{
    builder::Builder, md5_checksum::Md5Checksum, molecule_topology::MoleculeTopology, tag::Tag,
};

use super::{record, Record};

/// A SAM header reference sequence.
///
/// The reference sequence describes a sequence a read possibly mapped to. Both the reference
/// sequence name and length are guaranteed to be set.
///
/// A list of reference sequences creates a reference sequence dictionary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReferenceSequence {
    name: String,
    len: i32,
    alternative_locus: Option<String>,
    alternative_names: Option<String>,
    assemby_id: Option<String>,
    description: Option<String>,
    md5_checksum: Option<Md5Checksum>,
    species: Option<String>,
    molecule_topology: Option<MoleculeTopology>,
    uri: Option<String>,
    fields: HashMap<Tag, String>,
}

#[allow(clippy::len_without_is_empty)]
impl ReferenceSequence {
    /// Creates a SAM header reference sequence builder.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_sam::header::ReferenceSequence;
    /// let builder = ReferenceSequence::builder();
    /// ```
    pub fn builder() -> Builder {
        Builder::default()
    }

    /// Creates a reference sequence with a name and length.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_sam::header::ReferenceSequence;
    ///
    /// let reference_sequence = ReferenceSequence::new(String::from("sq0"), 13);
    ///
    /// assert_eq!(reference_sequence.name(), "sq0");
    /// assert_eq!(reference_sequence.len(), 13);
    /// ```
    pub fn new(name: String, len: i32) -> Self {
        Self {
            name,
            len,
            alternative_locus: None,
            alternative_names: None,
            assemby_id: None,
            description: None,
            md5_checksum: None,
            species: None,
            molecule_topology: None,
            uri: None,
            fields: HashMap::new(),
        }
    }

    /// Returns the reference sequence name.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_sam::header::ReferenceSequence;
    /// let mut reference_sequence = ReferenceSequence::new(String::from("sq0"), 13);
    /// assert_eq!(reference_sequence.name(), "sq0");
    /// ```
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns a mutable reference to the reference sequence name.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_sam::header::ReferenceSequence;
    ///
    /// let mut reference_sequence = ReferenceSequence::new(String::from("sq0"), 13);
    /// assert_eq!(reference_sequence.name(), "sq0");
    ///
    /// *reference_sequence.name_mut() = String::from("sq1");
    /// assert_eq!(reference_sequence.name(), "sq1");
    /// ```
    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }

    /// Returns the reference sequence length.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_sam::header::ReferenceSequence;
    /// let mut reference_sequence = ReferenceSequence::new(String::from("sq0"), 13);
    /// assert_eq!(reference_sequence.len(), 13);
    /// ```
    pub fn len(&self) -> i32 {
        self.len
    }

    /// Returns a mutable reference to the reference sequence name.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_sam::header::ReferenceSequence;
    ///
    /// let mut reference_sequence = ReferenceSequence::new(String::from("sq0"), 13);
    /// assert_eq!(reference_sequence.len(), 13);
    ///
    /// *reference_sequence.len_mut() = 8;
    /// assert_eq!(reference_sequence.len(), 8);
    /// ```
    pub fn len_mut(&mut self) -> &mut i32 {
        &mut self.len
    }

    /// Returns the alternative locus.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_sam::header::ReferenceSequence;
    /// let mut reference_sequence = ReferenceSequence::new(String::from("sq0"), 13);
    /// assert!(reference_sequence.alternative_locus().is_none());
    /// ```
    pub fn alternative_locus(&self) -> Option<&str> {
        self.alternative_locus.as_deref()
    }

    /// Returns the alternative names (aliases) of the reference sequence.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_sam::header::ReferenceSequence;
    /// let mut reference_sequence = ReferenceSequence::new(String::from("sq0"), 13);
    /// assert!(reference_sequence.alternative_names().is_none());
    /// ```
    pub fn alternative_names(&self) -> Option<&str> {
        self.alternative_names.as_deref()
    }

    /// Returns the genome assembly ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_sam::header::ReferenceSequence;
    /// let mut reference_sequence = ReferenceSequence::new(String::from("sq0"), 13);
    /// assert!(reference_sequence.assemby_id().is_none());
    /// ```
    pub fn assemby_id(&self) -> Option<&str> {
        self.assemby_id.as_deref()
    }

    /// Returns the description.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_sam::header::ReferenceSequence;
    /// let mut reference_sequence = ReferenceSequence::new(String::from("sq0"), 13);
    /// assert!(reference_sequence.description().is_none());
    /// ```
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Returns the MD5 checksum.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_sam::header::ReferenceSequence;
    /// let mut reference_sequence = ReferenceSequence::new(String::from("sq0"), 13);
    /// assert!(reference_sequence.md5_checksum().is_none());
    /// ```
    pub fn md5_checksum(&self) -> Option<Md5Checksum> {
        self.md5_checksum
    }

    /// Returns the species.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_sam::header::ReferenceSequence;
    /// let mut reference_sequence = ReferenceSequence::new(String::from("sq0"), 13);
    /// assert!(reference_sequence.species().is_none());
    /// ```
    pub fn species(&self) -> Option<&str> {
        self.species.as_deref()
    }

    /// Returns the molecule topology.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_sam::header::ReferenceSequence;
    /// let mut reference_sequence = ReferenceSequence::new(String::from("sq0"), 13);
    /// assert!(reference_sequence.molecule_topology().is_none());
    /// ```
    pub fn molecule_topology(&self) -> Option<MoleculeTopology> {
        self.molecule_topology
    }

    /// Returns the URI.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_sam::header::ReferenceSequence;
    /// let mut reference_sequence = ReferenceSequence::new(String::from("sq0"), 13);
    /// assert!(reference_sequence.uri().is_none());
    /// ```
    pub fn uri(&self) -> Option<&str> {
        self.uri.as_deref()
    }

    /// Returns the raw fields of the reference sequence.
    ///
    /// This includes any field that is not specially handled by the structure itself. For example,
    /// this will not include the name and length fields, as they are parsed and available as
    /// [`name`] and [`len`], respectively.
    ///
    /// [`name`]: #method.name
    /// [`len`]: #method.len
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_sam::header::{reference_sequence::Tag, ReferenceSequence};
    ///
    /// let mut reference_sequence = ReferenceSequence::new(String::from("sq0"), 13);
    /// reference_sequence.insert(Tag::Other(String::from("zn")), String::from("noodles"));
    ///
    /// let fields = reference_sequence.fields();
    ///
    /// assert_eq!(fields.len(), 1);
    /// assert_eq!(
    ///     fields.get(&Tag::Other(String::from("zn"))),
    ///     Some(&String::from("noodles"))
    /// );
    ///
    /// assert_eq!(fields.get(&Tag::Name), None);
    /// assert_eq!(reference_sequence.name(), "sq0");
    ///
    /// assert_eq!(fields.get(&Tag::Length), None);
    /// assert_eq!(reference_sequence.len(), 13);
    /// ```
    pub fn fields(&self) -> &HashMap<Tag, String> {
        &self.fields
    }

    /// Returns a reference to the raw field value mapped to the given key.
    ///
    /// This can only be used for fields with unparsed values. For example, [`name`] and [`len`]
    /// must be used instead of `get(Tag::Name)` and `get(Tag::Length)`, respectively.
    ///
    /// [`name`]: #method.name
    /// [`len`]: #method.len
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_sam::header::{reference_sequence::Tag, ReferenceSequence};
    ///
    /// let mut reference_sequence = ReferenceSequence::new(String::from("sq0"), 13);
    /// reference_sequence.insert(Tag::Other(String::from("zn")), String::from("noodles"));
    ///
    /// assert_eq!(
    ///     reference_sequence.get(&Tag::Other(String::from("zn"))),
    ///     Some(&String::from("noodles"))
    /// );
    /// assert_eq!(reference_sequence.get(&Tag::Name), None);
    /// ```
    pub fn get(&self, tag: &Tag) -> Option<&String> {
        self.fields.get(tag)
    }

    /// Inserts a tag-raw value pair into the reference sequence.
    ///
    /// This follows similar semantics to [`std::collections::HashMap::insert`].
    ///
    /// [`std::collections::HashMap::insert`]: https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html#method.insert
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_sam::header::{reference_sequence::Tag, ReferenceSequence};
    /// let mut reference_sequence = ReferenceSequence::new(String::from("sq0"), 13);
    /// reference_sequence.insert(Tag::Other(String::from("zn")), String::from("noodles"));
    /// ```
    pub fn insert(&mut self, tag: Tag, value: String) -> Option<String> {
        self.fields.insert(tag, value)
    }
}

impl fmt::Display for ReferenceSequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", record::Kind::ReferenceSequence)?;
        write!(f, "\t{}:{}", Tag::Name, self.name())?;
        write!(f, "\t{}:{}", Tag::Length, self.len())?;

        if let Some(alternative_locus) = self.alternative_locus() {
            write!(f, "\t{}:{}", Tag::AlternativeLocus, alternative_locus)?;
        }

        if let Some(alternative_names) = self.alternative_names() {
            write!(f, "\t{}:{}", Tag::AlternativeNames, alternative_names)?;
        }

        if let Some(assembly_id) = self.assemby_id() {
            write!(f, "\t{}:{}", Tag::AssemblyId, assembly_id)?;
        }

        if let Some(description) = self.description() {
            write!(f, "\t{}:{}", Tag::Description, description)?;
        }

        if let Some(md5_checksum) = self.md5_checksum() {
            write!(f, "\t{}:{}", Tag::Md5Checksum, md5_checksum)?;
        }

        if let Some(species) = self.species() {
            write!(f, "\t{}:{}", Tag::Species, species)?;
        }

        if let Some(molecule_topology) = self.molecule_topology() {
            write!(f, "\t{}:{}", Tag::MoleculeTopology, molecule_topology)?;
        }

        if let Some(uri) = self.uri() {
            write!(f, "\t{}:{}", Tag::Uri, uri)?;
        }

        for (tag, value) in &self.fields {
            write!(f, "\t{}:{}", tag, value)?;
        }

        Ok(())
    }
}

/// An error returned when a raw SAM header reference sequence fails to parse.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TryFromRecordError {
    /// The record is invalid.
    InvalidRecord,
    /// A required tag is missing.
    MissingRequiredTag(Tag),
    /// A tag is invalid.
    InvalidTag(tag::ParseError),
    /// The length tag (`LN`) has a invalid value.
    InvalidLength(num::ParseIntError),
    /// The MD5 checksum is invalid.
    InvalidMd5Checksum(md5_checksum::ParseError),
    /// The molecule topology is invalid.
    InvalidMoleculeTopology(molecule_topology::ParseError),
}

impl error::Error for TryFromRecordError {}

impl fmt::Display for TryFromRecordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRecord => f.write_str("invalid record"),
            Self::MissingRequiredTag(tag) => write!(f, "missing required tag: {:?}", tag),
            Self::InvalidTag(e) => write!(f, "{}", e),
            Self::InvalidLength(e) => write!(f, "invalid reference sequence length: {}", e),
            Self::InvalidMd5Checksum(e) => write!(f, "invalid MD5 checksum: {}", e),
            Self::InvalidMoleculeTopology(e) => write!(f, "invalid molecule topology: {}", e),
        }
    }
}

impl TryFrom<Record> for ReferenceSequence {
    type Error = TryFromRecordError;

    fn try_from(record: Record) -> Result<Self, Self::Error> {
        match record.into() {
            (record::Kind::ReferenceSequence, record::Value::Map(fields)) => parse_map(fields),
            _ => Err(TryFromRecordError::InvalidRecord),
        }
    }
}

fn parse_map(raw_fields: Vec<(String, String)>) -> Result<ReferenceSequence, TryFromRecordError> {
    let mut builder = ReferenceSequence::builder();

    let mut name = None;
    let mut len = None;

    for (raw_tag, value) in raw_fields {
        let tag = raw_tag.parse().map_err(TryFromRecordError::InvalidTag)?;

        builder = match tag {
            Tag::Name => {
                name = Some(value);
                builder
            }
            Tag::Length => {
                len = value
                    .parse()
                    .map(Some)
                    .map_err(TryFromRecordError::InvalidLength)?;
                builder
            }
            Tag::AlternativeLocus => builder.set_alternative_locus(value),
            Tag::AlternativeNames => builder.set_alternative_names(value),
            Tag::AssemblyId => builder.set_assembly_id(value),
            Tag::Description => builder.set_description(value),
            Tag::Md5Checksum => {
                let md5_checksum = value
                    .parse()
                    .map_err(TryFromRecordError::InvalidMd5Checksum)?;
                builder.set_md5_checksum(md5_checksum)
            }
            Tag::Species => builder.set_species(value),
            Tag::MoleculeTopology => {
                let molecule_topology = value
                    .parse()
                    .map_err(TryFromRecordError::InvalidMoleculeTopology)?;
                builder.set_molecule_topology(molecule_topology)
            }
            Tag::Uri => builder.set_uri(value),
            _ => builder.insert(tag, value),
        }
    }

    if let Some(n) = name {
        builder = builder.set_name(n);
    } else {
        return Err(TryFromRecordError::MissingRequiredTag(Tag::Name));
    }

    if let Some(l) = len {
        builder = builder.set_length(l);
    } else {
        return Err(TryFromRecordError::MissingRequiredTag(Tag::Length));
    }

    Ok(builder.build())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fmt() {
        let reference_sequence = ReferenceSequence::builder()
            .set_name("sq0")
            .set_length(13)
            .set_md5_checksum(Md5Checksum::from([
                0xd7, 0xeb, 0xa3, 0x11, 0x42, 0x1b, 0xbc, 0x9d, 0x3a, 0xda, 0x44, 0x70, 0x9d, 0xd6,
                0x15, 0x34,
            ]))
            .build();

        assert_eq!(
            reference_sequence.to_string(),
            "@SQ\tSN:sq0\tLN:13\tM5:d7eba311421bbc9d3ada44709dd61534"
        );
    }

    #[test]
    fn test_try_from_record_for_reference_sequence_with_invalid_record() {
        let record = Record::new(
            record::Kind::Comment,
            record::Value::String(String::from("noodles-sam")),
        );

        assert_eq!(
            ReferenceSequence::try_from(record),
            Err(TryFromRecordError::InvalidRecord)
        );
    }

    #[test]
    fn test_try_from_record_for_reference_sequence_with_missing_name() {
        let record = Record::new(
            record::Kind::ReferenceSequence,
            record::Value::Map(vec![
                (String::from("LN"), String::from("1")),
                (
                    String::from("M5"),
                    String::from("d7eba311421bbc9d3ada44709dd61534"),
                ),
            ]),
        );

        assert_eq!(
            ReferenceSequence::try_from(record),
            Err(TryFromRecordError::MissingRequiredTag(Tag::Name))
        );
    }

    #[test]
    fn test_try_from_record_for_reference_sequence_with_missing_length() {
        let record = Record::new(
            record::Kind::ReferenceSequence,
            record::Value::Map(vec![
                (String::from("SN"), String::from("sq0")),
                (
                    String::from("M5"),
                    String::from("d7eba311421bbc9d3ada44709dd61534"),
                ),
            ]),
        );

        assert_eq!(
            ReferenceSequence::try_from(record),
            Err(TryFromRecordError::MissingRequiredTag(Tag::Length))
        );
    }

    #[test]
    fn test_try_from_record_for_reference_sequence_with_missing_name_and_length() {
        let record = Record::new(
            record::Kind::ReferenceSequence,
            record::Value::Map(vec![(
                String::from("M5"),
                String::from("d7eba311421bbc9d3ada44709dd61534"),
            )]),
        );

        assert_eq!(
            ReferenceSequence::try_from(record),
            Err(TryFromRecordError::MissingRequiredTag(Tag::Name))
        );
    }

    #[test]
    fn test_try_from_record_for_reference_sequence_with_invalid_length() {
        let record = Record::new(
            record::Kind::ReferenceSequence,
            record::Value::Map(vec![(String::from("LN"), String::from("thirteen"))]),
        );

        assert!(matches!(
            ReferenceSequence::try_from(record),
            Err(TryFromRecordError::InvalidLength(_))
        ));
    }
}
