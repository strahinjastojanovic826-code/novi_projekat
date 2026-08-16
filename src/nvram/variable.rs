#[derive(Debug, Clone, PartialEq)]
pub enum NvramAttribute {
    NonVolatile,
    BootService,
    RuntimeService,
    ReadOnly,
}

#[derive(Debug, Clone)]
pub struct NvramVariable {
    pub name: String,
    pub value: Vec<u8>,
    pub attributes: Vec<NvramAttribute>,
}

impl NvramVariable {
    pub fn get_string_val(&self) -> String {
        String::from_utf8(self.value.clone()).unwrap_or_else(|_| "<Biparni Podatak>".into())
    }

    pub fn is_read_only(&self) -> bool {
        self.attributes.contains(&NvramAttribute::ReadOnly)
    }
}