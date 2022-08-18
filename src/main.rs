mod header_types;

use std::env;
use header_types::FILEHEADER;


fn main() -> Result<(),String>{
    let args:Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!();
        return Err(format!("usage: {} input output", args[0]));
    }

    let result = std::fs::read(args[1].clone());

    let input_data:Vec<u8>;

    match result {
        Ok(data) => {input_data = data;},
        Err(error) => {
            eprintln!();
            return Err(format!("error while opening file '{}': {}",args[1],error))
        }
    }
    
    if input_data.len() <= 54 || (input_data.len()-54)%3 != 0  {
        return Err(format!("'{}' is not a bmp image file",args[1]));
    }

    let mut file_header:FILEHEADER = FILEHEADER::new();

    

    file_header.bftype = u16::from_le_bytes(input_data[0..2].try_into().unwrap());
    if file_header.bftype != 19778 { //all BMP files start with 2 bytes equall to 19778
        return Err(format!("'{}' is not a bmp image file",args[1]));
    }


    return Ok(());
}


