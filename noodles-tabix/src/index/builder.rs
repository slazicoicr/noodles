use super::{format::CoordinateSystem, Format, Index, ReferenceSequence};

/// A tabix index builder.
pub struct Builder {
    format: Format,
    reference_sequence_name_index: usize,
    start_position_index: usize,
    end_position_index: usize,
    line_comment_prefix: u8,
    header_line_count: u32,
    reference_sequence_names: Vec<String>,
    reference_sequences: Vec<ReferenceSequence>,
    unmapped_read_count: Option<u64>,
}

impl Builder {
    /// Sets a tabix index format.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_tabix::{self as tabix, index::Format};
    /// let index = tabix::Index::builder().set_format(Format::Vcf).build();
    /// assert_eq!(index.format(), Format::Vcf);
    /// ```
    pub fn set_format(mut self, format: Format) -> Self {
        self.format = format;
        self
    }

    /// Sets a tabix index reference sequence name index.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_tabix as tabix;
    ///
    /// let index = tabix::Index::builder()
    ///     .set_reference_sequence_name_index(1)
    ///     .build();
    ///
    /// assert_eq!(index.reference_sequence_name_index(), 1);
    /// ```
    pub fn set_reference_sequence_name_index(
        mut self,
        reference_sequence_name_index: usize,
    ) -> Self {
        self.reference_sequence_name_index = reference_sequence_name_index;
        self
    }

    /// Sets a tabix index start position index.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_tabix as tabix;
    ///
    /// let index = tabix::Index::builder()
    ///     .set_start_position_index(4)
    ///     .build();
    ///
    /// assert_eq!(index.start_position_index(), 4);
    /// ```
    pub fn set_start_position_index(mut self, start_position_index: usize) -> Self {
        self.start_position_index = start_position_index;
        self
    }

    /// Sets a tabix index end position index.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_tabix as tabix;
    ///
    /// let index = tabix::Index::builder()
    ///     .set_end_position_index(5)
    ///     .build();
    ///
    /// assert_eq!(index.end_position_index(), 5);
    /// ```
    pub fn set_end_position_index(mut self, end_position_index: usize) -> Self {
        self.end_position_index = end_position_index;
        self
    }

    /// Sets a tabix index line comment prefix.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_tabix as tabix;
    ///
    /// let index = tabix::Index::builder()
    ///     .set_line_comment_prefix(b'#')
    ///     .build();
    ///
    /// assert_eq!(index.line_comment_prefix(), b'#');
    /// ```
    pub fn set_line_comment_prefix(mut self, line_comment_prefix: u8) -> Self {
        self.line_comment_prefix = line_comment_prefix;
        self
    }

    /// Sets a tabix index header line count.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_tabix as tabix;
    ///
    /// let index = tabix::Index::builder()
    ///     .set_header_line_count(0)
    ///     .build();
    ///
    /// assert_eq!(index.header_line_count(), 0);
    /// ```
    pub fn set_header_line_count(mut self, header_line_count: u32) -> Self {
        self.header_line_count = header_line_count;
        self
    }

    /// Sets tabix index reference sequence names.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_tabix as tabix;
    ///
    /// let index = tabix::Index::builder()
    ///     .set_reference_sequence_names(vec![String::from("sq0")])
    ///     .build();
    ///
    /// assert_eq!(index.reference_sequence_names(), [String::from("sq0")]);
    /// ```
    pub fn set_reference_sequence_names(mut self, reference_sequence_names: Vec<String>) -> Self {
        self.reference_sequence_names = reference_sequence_names;
        self
    }

    /// Sets tabix index reference sequences.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_tabix::{self as tabix, index::ReferenceSequence};
    ///
    /// let reference_sequences = vec![ReferenceSequence::new(Vec::new(), Vec::new())];
    ///
    /// let index = tabix::Index::builder()
    ///     .set_reference_sequences(reference_sequences)
    ///     .build();
    ///
    /// assert_eq!(index.reference_sequences().len(), 1);
    /// ```
    pub fn set_reference_sequences(mut self, reference_sequences: Vec<ReferenceSequence>) -> Self {
        self.reference_sequences = reference_sequences;
        self
    }

    /// Sets a tabix index unmapped read count.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_tabix as tabix;
    ///
    /// let index = tabix::Index::builder()
    ///     .set_unmapped_read_count(21)
    ///     .build();
    ///
    /// assert_eq!(index.unmapped_read_count(), Some(21));
    /// ```
    pub fn set_unmapped_read_count(mut self, unmapped_read_count: u64) -> Self {
        self.unmapped_read_count = Some(unmapped_read_count);
        self
    }

    /// Builds a tabix index.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_tabix as tabix;
    /// let index = tabix::Index::builder().build();
    /// ```
    pub fn build(self) -> Index {
        Index {
            format: self.format,
            reference_sequence_name_index: self.reference_sequence_name_index,
            start_position_index: self.start_position_index,
            end_position_index: self.end_position_index,
            line_comment_prefix: self.line_comment_prefix,
            header_line_count: self.header_line_count,
            reference_sequence_names: self.reference_sequence_names,
            reference_sequences: self.reference_sequences,
            unmapped_read_count: self.unmapped_read_count,
        }
    }
}

impl Default for Builder {
    fn default() -> Self {
        Builder {
            format: Format::Generic(CoordinateSystem::Gff),
            reference_sequence_name_index: 1,
            start_position_index: 4,
            end_position_index: 5,
            line_comment_prefix: b'#',
            header_line_count: 0,
            reference_sequence_names: Vec::new(),
            reference_sequences: Vec::new(),
            unmapped_read_count: None,
        }
    }
}
