extern crate num_complex;
extern crate rand;
extern crate wave_utils;
use wave_utils::MonoPcm;
use wave_utils::c_double;
use std::f64::consts::PI;

pub mod first;



fn sine_wave(pcm: &mut MonoPcm, f0: c_double, a: c_double, offset: usize, duration: usize) {
    /* サイン波 */
    let mut s: Vec<c_double> = (0..duration)
        .map(|n| (2.0 * PI * f0 * (n as f64) / (pcm.fs as f64)).sin() * a)
        .collect();

    /* フェード処理 */
    for n in 0..(pcm.fs as f64 * 0.01).ceil() as usize {
        s[n] *= n as c_double / (pcm.fs as f64 * 0.01);
        s[duration - n - 1] *= n as c_double / (pcm.fs as f64 * 0.01);
    }

    for n in 0..duration as usize {
        pcm.s[offset + n] += s[n];
    }
}

pub fn mult(i: usize, d: f64) -> usize {
    ((i as f64) * d) as usize
}




pub fn linear(initial_v: f64, final_v: f64, length: usize) -> Vec<f64> {
    (0..length)
        .map(|n| initial_v + (final_v - initial_v) * n as f64 / (length - 1) as f64)
        .collect()
}




#[allow(non_snake_case)]
pub fn determine_J(delta: f64) -> usize {
    let mut J = (3.1 / delta + 0.5) as usize - 1; /* 遅延器の数 */
    if J % 2 == 1 {
        J += 1; /* J+1が奇数になるように調整する */
    }
    return J;
}
