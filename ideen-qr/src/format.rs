pub enum ErrorLevel {
    L,
    M,
    Q,
    H,
}

pub enum Encoding {
    Numeric,
    Alphanumeric,
    Binary,
    Kanji,
}

impl Encoding {
    pub fn auto(data: &str) -> Self {
        if data.chars().all(|c| c.is_ascii_digit()) {
            Self::Numeric
        } else if data.chars().all(|c| c.is_ascii_alphanumeric()) {
            Self::Alphanumeric
        } else {
            Self::Binary
            // TODO: add kanji
        }
    }
}

pub struct Version(u8);

impl Version {
    pub fn new(number: u8) -> Option<Version> {
        match number {
            1..=40 => Some(Self(number)),
            _ => None,
        }
    }

    pub fn num(&self) -> u8 {
        self.0
    }

    pub fn dim_size(&self) -> u32 {
        self.num() as u32 * 4 + 17
    }
}

pub struct Format {
    pub version: Version,
    pub encoding: Encoding,
    pub error_level: ErrorLevel,
    pub mask_pattern: u8,
    pub empty_gap: u8,
}

impl Default for Format {
    fn default() -> Self {
        Self {
            version: Version(1),
            encoding: Encoding::Binary,
            error_level: ErrorLevel::L,
            mask_pattern: 0,
            empty_gap: 4,
        }
    }
}

impl Format {
    pub fn auto(data: &str) -> Self {
        Self {
            version: Version(1),
            encoding: Encoding::auto(data),
            ..Default::default()
        }
    }
}
