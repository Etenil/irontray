pub trait FromU16 {
    type Err;
    
    fn from_u16(num: u16) -> Result<Self, Self::Err>;
}

pub trait FromString {
    type Err;
    
    fn from_string(s: String) -> Result<Self, Self::Err>;
}