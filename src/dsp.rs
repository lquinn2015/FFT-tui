use num_complex::Complex;
use std::f32::consts::PI;

struct Sequence<T,U>
where U: Fn(T)->T
{
    x: T,
    f: U
}
impl<T: Copy, U: Fn(T)->T> Iterator for Sequence<T, U>
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let ret = Some(self.x);
        self.x = (self.f)(self.x);
        ret
    }
}
fn seq<T: Copy, U: Fn(T)->T>(x: T, f: U) -> Sequence<T,U>{
    Sequence{ x, f}
}

fn bit_reverse(num: u32, lg_n: u32) -> u32 {
    let mut rev : u32= 0; 
    for i in 0..lg_n {
        if (num & (1<<i)) != 0 {
            rev |= 1 << (lg_n -1 - i);
        }
    }
    rev
}

pub fn fft(sig: &mut Vec<Complex<f32>>, invert: bool) -> Result<(), String>{

    let n: u32 = sig.len() as u32;
    let mut lg_n = 0;
    while (1 << lg_n) < n {
        lg_n = lg_n + 1;
    }
    if 1 << lg_n != n {
        return Err("non power of two".to_string());
    }
    //println!("lg_n: {:?}", lg_n);
    //bit reverse we know the number of bits now reverse them
    for i in 0..n/2 {
        let brev = bit_reverse(i, lg_n);
        if i < brev {
            //println!("Swaping: {:?} and  {:?}", i, brev);
            sig.swap(i as usize, brev as usize);
        }
    }

    for order in seq(2 as usize, |x| x<<1).take_while(|&x| x <= n as usize) {
        //println!("Order: {}", order); 
        let angle: f32 = 2.0 * PI / order as f32 * if invert {-1.0} else {1.0};
        let step_angle: Complex<f32>  = Complex::new(angle.cos(), angle.sin());
        for i in seq(0 as usize, |x| x + order).take_while(|&x| x < n as usize) {
            let mut w: Complex<f32> = Complex::new(1.0, 0.0);
            for j in 0..order/2 {
                let u = sig[i+j];
                let v = sig[i+j + order/2] * w;
                sig[i+j] = u + v;
                sig[i+j+order/2] = u - v;
                //println!("u: {}, v: {}, w: {}, -w: {}, ", i+j, i+j+order/2, w, -w);
                //println!("Butterfly: f[{}]={}, f[{}]={}, u+v: {}, u-v: {}", i+j, u, i+j+order/2, v, u+v, u-v);
                w = w * step_angle; 
            }
        }
    }
    if invert {
        for i in 0..n {
            let i: usize = i as usize;
            sig[i] = sig[i] / n as f32;
        }
    }
    Ok(())
}
