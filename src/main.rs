mod header_types;

use std::{env, io::Write};
use std::fs::File;
use header_types::{FILEHEADER, INFOHEADER};


fn main() -> Result<(),String>{
    //read args
    let args:Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!();
        return Err(format!("usage: {} input output", args[0]));
    }

    //read file
    let result = std::fs::read(args[1].clone());

    let input_data:Vec<u8>;

    match result {
        Ok(data) => {input_data = data;},
        Err(error) => {
            return Err(format!("error while opening file '{}': {}",args[1],error))
        }
    }
    
    //check data
    if input_data.len() <= 54 {
        println!("{}",input_data.len());
        return Err(format!("'{}' is not a bmp image file, line:{}",args[1], line!()));
    }

    //store the data in data structors
    let mut file_header:FILEHEADER = FILEHEADER::new();
    let mut info_header:INFOHEADER = INFOHEADER::new();
    

    file_header.bftype = u16::from_le_bytes(input_data[0..2].try_into().unwrap());
    if file_header.bftype != 19778 { //all BMP files start with 2 bytes equall to 19778
        return Err(format!("'{}' is not a bmp image file, line:{}",args[1], line!()));
    }
    
    file_header.bfsize = u32::from_le_bytes(input_data[2..6].try_into().unwrap());

    if file_header.bfsize as usize != input_data.len() {
        return Err(format!("'{}' is not a bmp image file, line:{}",args[1], line!()));
    }
    
    file_header.bfoofbits = u32::from_le_bytes(input_data[10..14].try_into().unwrap());
    if file_header.bfoofbits != 54 {
        return Err(format!("'{}' is an illegable bmp image file, line:{}",args[1], line!()));
    }

    info_header.size = u32::from_le_bytes(input_data[14..18].try_into().unwrap());
    if info_header.size != 40 {
        return Err(format!("'{}' is an illegable bmp image file, line:{}",args[1], line!()));
    }

    info_header.width = i32::from_le_bytes(input_data[18..22].try_into().unwrap());
    info_header.height = i32::from_le_bytes(input_data[22..26].try_into().unwrap());
    info_header.planes = u16::from_le_bytes(input_data[26..28].try_into().unwrap());
    info_header.bits = u16::from_le_bytes(input_data[28..30].try_into().unwrap());
    info_header.compression = u32::from_le_bytes(input_data[30..34].try_into().unwrap());
    
    if info_header.compression != 0 {
        return Err(format!("'{}' is an illegable bmp image file, line:{}",args[1], line!()));
    }
    
    info_header.imagesize = u32::from_le_bytes(input_data[34..38].try_into().unwrap());
    info_header.xresolution = i32::from_le_bytes(input_data[38..42].try_into().unwrap());
    info_header.yresolution = i32::from_le_bytes(input_data[42..46].try_into().unwrap());
    info_header.ncolours = u32::from_le_bytes(input_data[46..50].try_into().unwrap());
    info_header.importantcolours = u32::from_le_bytes(input_data[50..54].try_into().unwrap());

    //convert colour data to black and white
    let mut bw_data:Vec<u8> = Vec::new();

    for index in (54..input_data.len()-2).step_by(3) {
        //0.2126*R + 0.7152*G + 0.0722*B , this is the formual that blender (an open-source 3D software) uses
        
        let b:u8 = input_data[index];
        let g:u8 = input_data[index+1];
        let r:u8 = input_data[index+2];
        let bw:u8 = ((r as f64 *0.2126) + (g as f64 *0.7152) + (b as f64 *0.0722)) as u8;
        bw_data.push(bw);
        bw_data.push(bw);
        bw_data.push(bw);
    }

    //write the data to the output file
    let mut out_file = match File::create(args[2].clone()) {
        Ok(output_file) => {
            output_file
        },
        Err(error) => {
            return Err(format!("{} at line:{}",error, line!())); 
        }
    }; 

    //write file header
    match out_file.write(&input_data[0..54]) {
        Ok(_) => {},
        Err(error) => {
            return Err(format!("{} at line:{}",error, line!())); 
        }
    }
    
    //write image data
    match out_file.write(&bw_data) {
        Ok(_) => {},
        Err(error) => {
            return Err(format!("{} at line:{}",error, line!())); 
        }
    }

    //write terminator character
    match out_file.write(&[0]) {
        Ok(_) => {},
        Err(error) => {
            return Err(format!("{} at line:{}",error, line!())); 
        }
    }

    //exit
    return Ok(());
}


