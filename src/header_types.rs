pub struct FILEHEADER{
    pub bftype:u16,
    pub bfsize:u32,
    pub bfoofbits:u32,
}

pub struct INFOHEADER{
    pub size:u32,
    pub width:i32,
    pub height:i32,
    pub planes:u16,
    pub bits:u16,
    pub compression:u32,
    pub imagesize:u32,
    pub xresolution:i32,
    pub yresolution:i32,
    pub ncolours:u32,
    pub importantcolours:u32,

}

impl FILEHEADER {
    pub fn new() -> FILEHEADER{
        return FILEHEADER { bftype: 0, bfsize: 0, bfoofbits: 0 };
    }
}

impl INFOHEADER {
    pub fn new() -> INFOHEADER{
        return INFOHEADER { size: 0, width: 0, height: 0, planes: 0, bits: 0, compression: 0, imagesize: 0,
            xresolution: 0, yresolution: 0, ncolours: 0, importantcolours: 0 };
    }
}


