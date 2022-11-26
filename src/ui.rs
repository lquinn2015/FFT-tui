
use std::f32::consts::PI;

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Chart, Dataset, Block, Borders, Axis, GraphType},
    style::{Color, Style},
    Frame, text::Span, symbols,
};

use num_complex::Complex;
use super::App;

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {

    let main_window = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black).fg(Color::Cyan))
        .title(format!("FFT"));

    f.render_widget(main_window, f.size());

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
        .split(f.size());

    let slot = app.x0..(app.x0+app.window as usize);
    let sig = &mut Vec::from_iter(app.samples[slot].iter().cloned());
    draw_fft(f, chunks[0], sig, app);
    
}

fn draw_fft<B: Backend>(f: &mut Frame<B>, window: Rect, signal: &mut  Vec<Complex<f32>>, app: &mut App){


    fft(signal, false).unwrap();
    let mut C2C: Vec<Complex<f32>> = signal.iter().copied().skip(signal.len()/2).chain(signal.iter().copied().take(signal.len()/2)).collect();
    let power_sig: Vec<(f64, f64)> = enumerate_PSD(&mut C2C, app.density)
        .into_iter().map(|x| (x.0 - app.panX, x.1)).collect();
    
    let ds = Dataset::default()
        .name(format!("FFT - PSD, window size {}", power_sig.len()))
        .marker(symbols::Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::default().fg(Color::Cyan).bg(Color::Black))
        .data(&power_sig);


    let psd = Chart::new(vec![ds])
        .block(Block::default().title("PSD - FFT "))
        .x_axis(Axis::default()
            .title(Span::styled("X - freq", Style::default().fg(Color::White)))
            .style(Style::default().fg(Color::White))
            .bounds([0.0, 130.0])
            .labels(["0.0", "1/2", "max"].iter().cloned().map(Span::from).collect())
        )
        .y_axis(Axis::default()
                .title(Span::styled("Y PSD", Style::default().fg(Color::White)))
                .style(Style::default().fg(Color::White))
                .bounds([-55.0, 2.0])
                .labels(["-45", "-20", "2"].iter().cloned().map(Span::from).collect())
        );
    f.render_widget(psd, window);
}

fn enumerate_PSD(sig: &mut Vec<Complex<f32>>, ptn2pixel: f64) -> Vec<(f64, f64)> {

    let sig2: Vec<f32> = sig.iter().copied().map(|x| (f32::sqrt(x.norm_sqr()) as f32)).collect();
    let sig_max = sig2.iter().copied().reduce(f32::max).unwrap();
    let stream: Vec<(f64, f64)> = sig2.into_iter().enumerate()
        .map(|x| ((x.0 as f64 * ptn2pixel), 20.0*(x.1 / sig_max).log10() as f64)).collect();
    //for x in stream.iter() {
    //    print!("( {:?}), ", x);
    //} println!();

    stream
}
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

fn bit_reverse(num: u32, n: u32) -> u32 {
    let mut rev : u32= 0; 
    for i in seq(1, |x| x<<1).take_while(|&x| x < n)  {
        if num & i != 0 {
          rev = rev ^ 1;
        }
        rev = rev << 1;
    }
    rev
}

fn fft(sig: &mut Vec<Complex<f32>>, invert: bool) -> Result<(), String>{

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
