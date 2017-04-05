#[macro_use]
extern crate nom;
extern crate image;

use nom::{le_u8, le_u32};

use image::Rgb;

use std::io::prelude::*;
use std::io::BufReader;
use std::io;
use std::fs::File;

mod grid;
use grid::Grid;

// A state change is described by 4 integers:
// Timestamp in seconds
// X
// Y
// Color id (see table at my older post)


// COlors:
// code	color
// 0	#FFFFFF
// 1	#E4E4E4
// 2	#888888
// 3	#222222
// 4	#FFA7D1
// 5	#E50000
// 6	#E59500
// 7	#A06A42
// 8	#E5D900
// 9	#94E044
// 10	#02BE01
// 11	#00D3DD
// 12	#0083C7
// 13	#0000EA
// 14	#CF6EE4
// 15	#820080
#[derive(Debug)]
struct StateChange {
    timestamp: u32,
    x: u32,
    y: u32,
    color: u8,
}

named!(state_change( &[u8] ) -> StateChange,
    do_parse!(
        timestamp: le_u32 >>
        x:         le_u32 >>
        y:         le_u32 >>
        color:     le_u8 >>
        skip:         take!(3) >>
        (StateChange{timestamp, x, y, color})
    )
);

fn mk_reader(filename: &str) -> io::Result<BufReader<File>> {
    let f = File::open(filename)?;
    Ok(BufReader::new(f))
}

fn color_code_to_hex(color: u8) -> u32 {
    match color {
        0  => 0xFFFFFF,
        1  => 0xE4E4E4,
        2  => 0x888888,
        3  => 0x222222,
        4  => 0xFFA7D1,
        5  => 0xE50000,
        6  => 0xE59500,
        7  => 0xA06A42,
        8  => 0xE5D900,
        9  => 0x94E044,
        10 => 0x02BE01,
        11 => 0x00D3DD,
        12 => 0x0083C7,
        13 => 0x0000EA,
        14 => 0xCF6EE4,
        15 => 0x820080,
        _ =>  panic!("out of bounds color code")
    }
}

fn color_to_rgb(color: u8) -> Rgb<u8> {
    let hex_color = color_code_to_hex(color);
    let bytes = unsafe {
        std::mem::transmute::<u32, [u8; 4]>(hex_color)
    };
    Rgb{data: [ bytes[2], bytes[1], bytes[0] ]}
}


fn main() {
    let mut reader = mk_reader("diffs.bin").expect("Failed getting reader");
    let mut buffer = [0; 4000];
    
    let mut board = Grid::new();

    loop {
        let mut progress = 0;
        let bytes_read = match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(bytes_read) => bytes_read,
            Err(_) => panic!("read fail"),
        };
        
        let mut go = true;
        let mut rest_input = &buffer[progress..bytes_read];
        while go {
            let res = state_change(rest_input);
            use nom::IResult::*;
            match res {
                Done(rest, state_change) => {
                    if rest.len() == 0 {
                        go = false;
                    }
                    progress += 4;
                    board.update_index(state_change.x, state_change.y, state_change.color);
                    rest_input = rest;
                },
                Error(_) => panic!("error"),
                Incomplete(_) => {
                    panic!("should never happen, because we always read 4 bytes")
                },
            }
        }
    }

    let mut imgbuf: image::ImageBuffer<Rgb<u8>,_> = image::ImageBuffer::new(999, 999);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = color_to_rgb(board.get(x,y))
    }
    imgbuf.save("out.png").expect("could not save");
}
