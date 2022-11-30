use super::dsp;

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


    dsp::fft(signal, false).unwrap();
    let mut C2C: Vec<Complex<f32>> = signal.iter().copied().skip(signal.len()/2).chain(signal.iter().copied().take(signal.len()/2)).collect();
    let power_sig: Vec<(f64, f64)> = enumerate_PSD(&mut C2C, app.density)
        .into_iter().map(|x| (x.0 - app.panX, x.1)).collect();
    
    let ds = Dataset::default()
        .name(format!("FFT - PSD, window size {}", power_sig.len()))
        .marker(symbols::Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::default().fg(Color::Cyan).bg(Color::Black))
        .data(&power_sig);


    let min_freq = app.panX;
    let max_freq = app.panX + window.width as f64 / app.density as f64;

    let x_labels = [format!("{}", min_freq), format!("{}", max_freq) ];

    let psd = Chart::new(vec![ds])
        .block(Block::default().title("PSD - FFT "))
        .x_axis(Axis::default()
            .title(Span::styled("X - freq", Style::default().fg(Color::White)))
            .style(Style::default().fg(Color::White))
            .bounds([0.0, 130.0])
            .labels(x_labels.iter().cloned().map(Span::from).collect())
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
