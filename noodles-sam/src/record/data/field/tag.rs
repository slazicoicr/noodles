use std::{error, fmt, str::FromStr};

const LEN: usize = 2;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Tag {
    MinMappingQuality,
    AlignmentScore,
    SampleBarcodeSequence,
    BaseAlignmentQualityOffsets,
    OriginalUmiQualityScores,
    CellBarcodeId,
    NextHitReferenceSequenceName,
    Cigar,
    ColorEditDistance,
    Comment,
    NextHitPosition,
    ColarQualityScores,
    CellBarcodeSequence,
    ColorSequence,
    CompleteReadAnnotations,
    CellBarcodeQualityScores,
    NextHitSequence,
    SegmentIndex,
    SegmentSuffix,
    AlterantiveSequence,
    ReservedGC,
    ReservedGQ,
    ReservedGS,
    PerfectHitCount,
    OneDifferenceHitCount,
    TwoDifferenceHitCount,
    HitIndex,
    TotalHitCount,
    Library,
    MateCigar,
    MismatchedPositions,
    ReservedMF,
    UmiId,
    MateMappingQuality,
    AlignmentHitCount,
    EditDistance,
    OriginalAlignment,
    OriginalCigar,
    OriginalPosition,
    OriginalQualityScores,
    OriginalUmiBarcodeSequence,
    Program,
    PaddedReadAnnotations,
    PlatformUnit,
    MateQualityScores,
    SampleBarcodeQualityScores,
    UmiQualityScores,
    MateSequence,
    ReadGroup,
    ReservedRT,
    UmiSequence,
    ReservedS2,
    OtherAlignments,
    TemplateMappingQuality,
    ReservedSQ,
    SegmentCount,
    TranscriptStrand,
    NextHitQualityScores,
    SegmentLikelihood,
    Other(String),
}

impl AsRef<str> for Tag {
    fn as_ref(&self) -> &str {
        match self {
            Self::MinMappingQuality => "AM",
            Self::AlignmentScore => "AS",
            Self::SampleBarcodeSequence => "BC",
            Self::BaseAlignmentQualityOffsets => "BQ",
            Self::OriginalUmiQualityScores => "BZ",
            Self::CellBarcodeId => "CB",
            Self::NextHitReferenceSequenceName => "CC",
            Self::Cigar => "CG",
            Self::ColorEditDistance => "CM",
            Self::Comment => "CO",
            Self::NextHitPosition => "CP",
            Self::ColarQualityScores => "CQ",
            Self::CellBarcodeSequence => "CR",
            Self::ColorSequence => "CS",
            Self::CompleteReadAnnotations => "CT",
            Self::CellBarcodeQualityScores => "CY",
            Self::NextHitSequence => "E2",
            Self::SegmentIndex => "FI",
            Self::SegmentSuffix => "FS",
            Self::AlterantiveSequence => "FZ",
            Self::ReservedGC => "GC",
            Self::ReservedGQ => "GQ",
            Self::ReservedGS => "GS",
            Self::PerfectHitCount => "HO",
            Self::OneDifferenceHitCount => "H1",
            Self::TwoDifferenceHitCount => "H2",
            Self::HitIndex => "HI",
            Self::TotalHitCount => "IH",
            Self::Library => "LB",
            Self::MateCigar => "MC",
            Self::MismatchedPositions => "MD",
            Self::ReservedMF => "MF",
            Self::UmiId => "MI",
            Self::MateMappingQuality => "MQ",
            Self::AlignmentHitCount => "NH",
            Self::EditDistance => "NM",
            Self::OriginalAlignment => "OA",
            Self::OriginalCigar => "OC",
            Self::OriginalPosition => "OP",
            Self::OriginalQualityScores => "OQ",
            Self::OriginalUmiBarcodeSequence => "OX",
            Self::Program => "PG",
            Self::PaddedReadAnnotations => "PT",
            Self::PlatformUnit => "PU",
            Self::MateQualityScores => "Q2",
            Self::SampleBarcodeQualityScores => "QT",
            Self::UmiQualityScores => "QX",
            Self::MateSequence => "R2",
            Self::ReadGroup => "RG",
            Self::ReservedRT => "RT",
            Self::UmiSequence => "RX",
            Self::ReservedS2 => "S2",
            Self::OtherAlignments => "SA",
            Self::TemplateMappingQuality => "SM",
            Self::ReservedSQ => "SQ",
            Self::SegmentCount => "TC",
            Self::TranscriptStrand => "TS",
            Self::NextHitQualityScores => "U2",
            Self::SegmentLikelihood => "UQ",
            Self::Other(tag) => tag,
        }
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_ref())
    }
}

#[derive(Debug)]
pub struct ParseError(String);

impl error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid data tag: {}", self.0)
    }
}

impl FromStr for Tag {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AM" => Ok(Self::MinMappingQuality),
            "AS" => Ok(Self::AlignmentScore),
            "BC" => Ok(Self::SampleBarcodeSequence),
            "BQ" => Ok(Self::BaseAlignmentQualityOffsets),
            "BZ" => Ok(Self::OriginalUmiQualityScores),
            "CB" => Ok(Self::CellBarcodeId),
            "CC" => Ok(Self::NextHitReferenceSequenceName),
            "CG" => Ok(Self::Cigar),
            "CM" => Ok(Self::ColorEditDistance),
            "CO" => Ok(Self::Comment),
            "CP" => Ok(Self::NextHitPosition),
            "CQ" => Ok(Self::ColarQualityScores),
            "CR" => Ok(Self::CellBarcodeSequence),
            "CS" => Ok(Self::ColorSequence),
            "CT" => Ok(Self::CompleteReadAnnotations),
            "CY" => Ok(Self::CellBarcodeQualityScores),
            "E2" => Ok(Self::NextHitSequence),
            "FI" => Ok(Self::SegmentIndex),
            "FS" => Ok(Self::SegmentSuffix),
            "FZ" => Ok(Self::AlterantiveSequence),
            "GC" => Ok(Self::ReservedGC),
            "GQ" => Ok(Self::ReservedGQ),
            "GS" => Ok(Self::ReservedGS),
            "HO" => Ok(Self::PerfectHitCount),
            "H1" => Ok(Self::OneDifferenceHitCount),
            "H2" => Ok(Self::TwoDifferenceHitCount),
            "HI" => Ok(Self::HitIndex),
            "IH" => Ok(Self::TotalHitCount),
            "LB" => Ok(Self::Library),
            "MC" => Ok(Self::MateCigar),
            "MD" => Ok(Self::MismatchedPositions),
            "MF" => Ok(Self::ReservedMF),
            "MI" => Ok(Self::UmiId),
            "MQ" => Ok(Self::MateMappingQuality),
            "NH" => Ok(Self::AlignmentHitCount),
            "NM" => Ok(Self::EditDistance),
            "OA" => Ok(Self::OriginalAlignment),
            "OC" => Ok(Self::OriginalCigar),
            "OP" => Ok(Self::OriginalPosition),
            "OQ" => Ok(Self::OriginalQualityScores),
            "OX" => Ok(Self::OriginalUmiBarcodeSequence),
            "PG" => Ok(Self::Program),
            "PT" => Ok(Self::PaddedReadAnnotations),
            "PU" => Ok(Self::PlatformUnit),
            "Q2" => Ok(Self::MateQualityScores),
            "QT" => Ok(Self::SampleBarcodeQualityScores),
            "QX" => Ok(Self::UmiQualityScores),
            "R2" => Ok(Self::MateSequence),
            "RG" => Ok(Self::ReadGroup),
            "RT" => Ok(Self::ReservedRT),
            "RX" => Ok(Self::UmiSequence),
            "S2" => Ok(Self::ReservedS2),
            "SA" => Ok(Self::OtherAlignments),
            "SM" => Ok(Self::TemplateMappingQuality),
            "SQ" => Ok(Self::ReservedSQ),
            "TC" => Ok(Self::SegmentCount),
            "TS" => Ok(Self::TranscriptStrand),
            "U2" => Ok(Self::NextHitQualityScores),
            "UQ" => Ok(Self::SegmentLikelihood),
            _ => {
                if s.len() == LEN {
                    Ok(Tag::Other(s.into()))
                } else {
                    Err(ParseError(s.into()))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fmt() {
        assert_eq!(Tag::MinMappingQuality.to_string(), "AM");
        assert_eq!(Tag::AlignmentScore.to_string(), "AS");
        assert_eq!(Tag::SampleBarcodeSequence.to_string(), "BC");
        assert_eq!(Tag::BaseAlignmentQualityOffsets.to_string(), "BQ");
        assert_eq!(Tag::OriginalUmiQualityScores.to_string(), "BZ");
        assert_eq!(Tag::CellBarcodeId.to_string(), "CB");
        assert_eq!(Tag::NextHitReferenceSequenceName.to_string(), "CC");
        assert_eq!(Tag::Cigar.to_string(), "CG");
        assert_eq!(Tag::ColorEditDistance.to_string(), "CM");
        assert_eq!(Tag::Comment.to_string(), "CO");
        assert_eq!(Tag::NextHitPosition.to_string(), "CP");
        assert_eq!(Tag::ColarQualityScores.to_string(), "CQ");
        assert_eq!(Tag::CellBarcodeSequence.to_string(), "CR");
        assert_eq!(Tag::ColorSequence.to_string(), "CS");
        assert_eq!(Tag::CompleteReadAnnotations.to_string(), "CT");
        assert_eq!(Tag::CellBarcodeQualityScores.to_string(), "CY");
        assert_eq!(Tag::NextHitSequence.to_string(), "E2");
        assert_eq!(Tag::SegmentIndex.to_string(), "FI");
        assert_eq!(Tag::SegmentSuffix.to_string(), "FS");
        assert_eq!(Tag::AlterantiveSequence.to_string(), "FZ");
        assert_eq!(Tag::ReservedGC.to_string(), "GC");
        assert_eq!(Tag::ReservedGQ.to_string(), "GQ");
        assert_eq!(Tag::ReservedGS.to_string(), "GS");
        assert_eq!(Tag::PerfectHitCount.to_string(), "HO");
        assert_eq!(Tag::OneDifferenceHitCount.to_string(), "H1");
        assert_eq!(Tag::TwoDifferenceHitCount.to_string(), "H2");
        assert_eq!(Tag::HitIndex.to_string(), "HI");
        assert_eq!(Tag::TotalHitCount.to_string(), "IH");
        assert_eq!(Tag::Library.to_string(), "LB");
        assert_eq!(Tag::MateCigar.to_string(), "MC");
        assert_eq!(Tag::MismatchedPositions.to_string(), "MD");
        assert_eq!(Tag::ReservedMF.to_string(), "MF");
        assert_eq!(Tag::UmiId.to_string(), "MI");
        assert_eq!(Tag::MateMappingQuality.to_string(), "MQ");
        assert_eq!(Tag::AlignmentHitCount.to_string(), "NH");
        assert_eq!(Tag::EditDistance.to_string(), "NM");
        assert_eq!(Tag::OriginalAlignment.to_string(), "OA");
        assert_eq!(Tag::OriginalCigar.to_string(), "OC");
        assert_eq!(Tag::OriginalPosition.to_string(), "OP");
        assert_eq!(Tag::OriginalQualityScores.to_string(), "OQ");
        assert_eq!(Tag::OriginalUmiBarcodeSequence.to_string(), "OX");
        assert_eq!(Tag::Program.to_string(), "PG");
        assert_eq!(Tag::PaddedReadAnnotations.to_string(), "PT");
        assert_eq!(Tag::PlatformUnit.to_string(), "PU");
        assert_eq!(Tag::MateQualityScores.to_string(), "Q2");
        assert_eq!(Tag::SampleBarcodeQualityScores.to_string(), "QT");
        assert_eq!(Tag::UmiQualityScores.to_string(), "QX");
        assert_eq!(Tag::MateSequence.to_string(), "R2");
        assert_eq!(Tag::ReadGroup.to_string(), "RG");
        assert_eq!(Tag::ReservedRT.to_string(), "RT");
        assert_eq!(Tag::UmiSequence.to_string(), "RX");
        assert_eq!(Tag::ReservedS2.to_string(), "S2");
        assert_eq!(Tag::OtherAlignments.to_string(), "SA");
        assert_eq!(Tag::TemplateMappingQuality.to_string(), "SM");
        assert_eq!(Tag::ReservedSQ.to_string(), "SQ");
        assert_eq!(Tag::SegmentCount.to_string(), "TC");
        assert_eq!(Tag::TranscriptStrand.to_string(), "TS");
        assert_eq!(Tag::NextHitQualityScores.to_string(), "U2");
        assert_eq!(Tag::SegmentLikelihood.to_string(), "UQ");
        assert_eq!(Tag::Other(String::from("ZN")).to_string(), "ZN");
    }

    #[test]
    fn test_from_str() -> Result<(), ParseError> {
        assert_eq!("AM".parse::<Tag>()?, Tag::MinMappingQuality);
        assert_eq!("AS".parse::<Tag>()?, Tag::AlignmentScore);
        assert_eq!("BC".parse::<Tag>()?, Tag::SampleBarcodeSequence);
        assert_eq!("BQ".parse::<Tag>()?, Tag::BaseAlignmentQualityOffsets);
        assert_eq!("BZ".parse::<Tag>()?, Tag::OriginalUmiQualityScores);
        assert_eq!("CB".parse::<Tag>()?, Tag::CellBarcodeId);
        assert_eq!("CC".parse::<Tag>()?, Tag::NextHitReferenceSequenceName);
        assert_eq!("CG".parse::<Tag>()?, Tag::Cigar);
        assert_eq!("CM".parse::<Tag>()?, Tag::ColorEditDistance);
        assert_eq!("CO".parse::<Tag>()?, Tag::Comment);
        assert_eq!("CP".parse::<Tag>()?, Tag::NextHitPosition);
        assert_eq!("CQ".parse::<Tag>()?, Tag::ColarQualityScores);
        assert_eq!("CR".parse::<Tag>()?, Tag::CellBarcodeSequence);
        assert_eq!("CS".parse::<Tag>()?, Tag::ColorSequence);
        assert_eq!("CT".parse::<Tag>()?, Tag::CompleteReadAnnotations);
        assert_eq!("CY".parse::<Tag>()?, Tag::CellBarcodeQualityScores);
        assert_eq!("E2".parse::<Tag>()?, Tag::NextHitSequence);
        assert_eq!("FI".parse::<Tag>()?, Tag::SegmentIndex);
        assert_eq!("FS".parse::<Tag>()?, Tag::SegmentSuffix);
        assert_eq!("FZ".parse::<Tag>()?, Tag::AlterantiveSequence);
        assert_eq!("GC".parse::<Tag>()?, Tag::ReservedGC);
        assert_eq!("GQ".parse::<Tag>()?, Tag::ReservedGQ);
        assert_eq!("GS".parse::<Tag>()?, Tag::ReservedGS);
        assert_eq!("HO".parse::<Tag>()?, Tag::PerfectHitCount);
        assert_eq!("H1".parse::<Tag>()?, Tag::OneDifferenceHitCount);
        assert_eq!("H2".parse::<Tag>()?, Tag::TwoDifferenceHitCount);
        assert_eq!("HI".parse::<Tag>()?, Tag::HitIndex);
        assert_eq!("IH".parse::<Tag>()?, Tag::TotalHitCount);
        assert_eq!("LB".parse::<Tag>()?, Tag::Library);
        assert_eq!("MC".parse::<Tag>()?, Tag::MateCigar);
        assert_eq!("MD".parse::<Tag>()?, Tag::MismatchedPositions);
        assert_eq!("MF".parse::<Tag>()?, Tag::ReservedMF);
        assert_eq!("MI".parse::<Tag>()?, Tag::UmiId);
        assert_eq!("MQ".parse::<Tag>()?, Tag::MateMappingQuality);
        assert_eq!("NH".parse::<Tag>()?, Tag::AlignmentHitCount);
        assert_eq!("NM".parse::<Tag>()?, Tag::EditDistance);
        assert_eq!("OA".parse::<Tag>()?, Tag::OriginalAlignment);
        assert_eq!("OC".parse::<Tag>()?, Tag::OriginalCigar);
        assert_eq!("OP".parse::<Tag>()?, Tag::OriginalPosition);
        assert_eq!("OQ".parse::<Tag>()?, Tag::OriginalQualityScores);
        assert_eq!("OX".parse::<Tag>()?, Tag::OriginalUmiBarcodeSequence);
        assert_eq!("PG".parse::<Tag>()?, Tag::Program);
        assert_eq!("PT".parse::<Tag>()?, Tag::PaddedReadAnnotations);
        assert_eq!("PU".parse::<Tag>()?, Tag::PlatformUnit);
        assert_eq!("Q2".parse::<Tag>()?, Tag::MateQualityScores);
        assert_eq!("QT".parse::<Tag>()?, Tag::SampleBarcodeQualityScores);
        assert_eq!("QX".parse::<Tag>()?, Tag::UmiQualityScores);
        assert_eq!("R2".parse::<Tag>()?, Tag::MateSequence);
        assert_eq!("RG".parse::<Tag>()?, Tag::ReadGroup);
        assert_eq!("RT".parse::<Tag>()?, Tag::ReservedRT);
        assert_eq!("RX".parse::<Tag>()?, Tag::UmiSequence);
        assert_eq!("S2".parse::<Tag>()?, Tag::ReservedS2);
        assert_eq!("SA".parse::<Tag>()?, Tag::OtherAlignments);
        assert_eq!("SM".parse::<Tag>()?, Tag::TemplateMappingQuality);
        assert_eq!("SQ".parse::<Tag>()?, Tag::ReservedSQ);
        assert_eq!("TC".parse::<Tag>()?, Tag::SegmentCount);
        assert_eq!("TS".parse::<Tag>()?, Tag::TranscriptStrand);
        assert_eq!("U2".parse::<Tag>()?, Tag::NextHitQualityScores);
        assert_eq!("UQ".parse::<Tag>()?, Tag::SegmentLikelihood);
        assert_eq!("ZN".parse::<Tag>()?, Tag::Other(String::from("ZN")));

        assert!("".parse::<Tag>().is_err());
        assert!("R".parse::<Tag>().is_err());
        assert!("RGP".parse::<Tag>().is_err());
        assert!("RGRP".parse::<Tag>().is_err());

        Ok(())
    }
}