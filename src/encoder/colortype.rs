use crate::decoder::PhotometricInterpretation;

/// Trait for different colortypes that can be encoded.
pub trait ColorType {
    /// The type of each sample of this colortype
    type Inner: super::TiffValue;
    /// The value of the tiff tag `PhotometricInterpretation`
    const TIFF_VALUE: PhotometricInterpretation;
    /// The value of the tiff tag `BitsPerSample`
    fn bits_per_sample() -> Vec<u16>;
}

pub struct Gray8;
impl ColorType for Gray8 {
    type Inner = u8;
    const TIFF_VALUE: PhotometricInterpretation = PhotometricInterpretation::BlackIsZero;
    fn bits_per_sample() -> Vec<u16> {
        vec![8]
    }
}

pub struct Gray16;
impl ColorType for Gray16 {
    type Inner = u16;
    const TIFF_VALUE: PhotometricInterpretation = PhotometricInterpretation::BlackIsZero;
    fn bits_per_sample() -> Vec<u16> {
        vec![16]
    }
}

pub struct RGB8;
impl ColorType for RGB8 {
    type Inner = u8;
    const TIFF_VALUE: PhotometricInterpretation = PhotometricInterpretation::RGB;
    fn bits_per_sample() -> Vec<u16> {
        vec![8,8,8]
    }
}

pub struct RGB16;
impl ColorType for RGB16 {
    type Inner = u16;
    const TIFF_VALUE: PhotometricInterpretation = PhotometricInterpretation::RGB;
    fn bits_per_sample() -> Vec<u16> {
        vec![16,16,16]
    }
}

pub struct RGBA8;
impl ColorType for RGBA8 {
    type Inner = u8;
    const TIFF_VALUE: PhotometricInterpretation = PhotometricInterpretation::RGB;
    fn bits_per_sample() -> Vec<u16> {
        vec![8,8,8,8]
    }
}

pub struct RGBA16;
impl ColorType for RGBA16 {
    type Inner = u16;
    const TIFF_VALUE: PhotometricInterpretation = PhotometricInterpretation::RGB;
    fn bits_per_sample() -> Vec<u16> {
        vec![16,16,16,16]
    }
}

pub struct CMYK8;
impl ColorType for CMYK8 {
    type Inner = u8;
    const TIFF_VALUE: PhotometricInterpretation = PhotometricInterpretation::CMYK;
    fn bits_per_sample() -> Vec<u16> {
        vec![8,8,8,8]
    }
}
