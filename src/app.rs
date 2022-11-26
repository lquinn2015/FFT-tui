
use num_complex::Complex;
use std::{fs::File, io::prelude::*};

pub struct App {
    pub samples: Vec<Complex<f32>>,
    pub x0: usize,
    pub window: u32,
    pub should_quit: bool,
    pub density: u32,
}

impl App {
    pub fn new(data_file: String) -> Self {
        let mut f  = File::open(data_file).expect("File not found");
        let mut buf: Vec<u8> = Vec::new();
        f.read_to_end(&mut buf).expect("IO failed");
        
        let mut x: Vec<Complex<f32>> = Vec::new();
        for s in buf.iter().step_by(2).copied().zip(buf.iter().step_by(2).copied()) {
            x.push(Complex::new(s.0 as f32 - 127.5, s.1 as f32 - 127.5))
        }
        App {samples: x, x0: 0, window: 4096, should_quit: false, density: 1}
    }

    pub fn set_x0(&mut self, amt: i32) {
        let x0n: i32 = self.x0 as i32;
        let x0p = x0n + amt;
        if x0p >= 0 {
            self.x0 = x0p as usize;
        }

    }
    
    pub fn on_tick() {
    }
}



