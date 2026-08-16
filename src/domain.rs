#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum QuquatVal {
    Q00 = 0b00,
    Q01 = 0b01,
    Q10 = 0b10,
    Q11 = 0b11,
}

impl QuquatVal {
    pub fn next(self) -> Self {
        match self {
            Self::Q00 => Self::Q01,
            Self::Q01 => Self::Q10,
            Self::Q10 => Self::Q11,
            Self::Q11 => Self::Q00,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Q00 => "|00⟩",
            Self::Q01 => "|01⟩",
            Self::Q10 => "|10⟩",
            Self::Q11 => "|11⟩",
        }
    }
}

//Ala sam ja lud sta sam smislio