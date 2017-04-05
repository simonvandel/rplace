
use image::Rgb;

#[derive(Debug)]
pub struct Grid {
    pub buffer: Vec<u8>
}

impl Grid {
    pub fn new() -> Self {
        let buffer = vec![0; 1000*1000];
        
        Grid {buffer: buffer }
    }

    pub fn update_index(&mut self, x:u32, y:u32, value: u8) {
        let index = get_index(x, y);
        *(self.buffer.get_mut(index).expect("index")) = value;
    }

    pub fn get(&self, x:u32, y:u32) -> u8 {
        let index = get_index(x, y);
        self.buffer[index]
    }
}

fn get_index(x: u32, y: u32) -> usize {
    (x * 999 + y) as usize
}