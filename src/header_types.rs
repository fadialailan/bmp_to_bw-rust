pub struct FILEHEADER{
    pub bftype:u16,
    pub bfsize:u32,
    pub bfoofbits:u32,
}

impl FILEHEADER {
    pub fn new() -> FILEHEADER{
        return FILEHEADER { bftype: 0, bfsize: 0, bfoofbits: 0 };
    }
}

