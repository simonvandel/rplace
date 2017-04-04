
use image::Rgb;

#[derive(Debug)]
pub struct Grid {
    pub buffer: Vec<Rgb<u8>>
}

impl Grid {
    pub fn new() -> Self {
        let default_subpixel = Rgb{data: [0u8,0,0]};
        let buffer = vec![default_subpixel; 9999*9999];
        
        Grid {buffer: buffer }
    }

    pub fn update_index(&mut self, x:u32, y:u32, value: Rgb<u8>) {
        let index = get_index(x, y);
        *(self.buffer.get_mut(index).expect("index")) = value;
    }

    pub fn get(&self, x:u32, y:u32) -> &Rgb<u8> {
        let index = get_index(x, y);
        &self.buffer[index]
    }
}

fn get_index(x: u32, y: u32) -> usize {
    (x * 9999 + y) as usize
}