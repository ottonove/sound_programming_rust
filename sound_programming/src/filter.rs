use sinc;
use std::f64::consts::PI;
use libc::c_double;
use libc::c_int;

#[link(name = "fir_filter")]
extern {
	//pub fn FIR_LPF(fe: c_double, J: c_int, b: *mut c_double, w: *mut c_double);
	pub fn FIR_HPF(fe: c_double, J: c_int, b: *mut c_double, w: *mut c_double);
	pub fn FIR_BPF(fe1: c_double, fe2: c_double, J: c_int, b: *mut c_double, w: *mut c_double);
	pub fn FIR_BEF(fe1: c_double, fe2: c_double, J: c_int, b: *mut c_double, w: *mut c_double);
	//*pub*/ fn FIR_filtering(x: *const c_double, y: *mut c_double, L: c_int, b: *mut c_double, J: c_int);
}

#[allow(non_snake_case)]
pub fn safe_FIR_LPF(fe: c_double, J: usize, b: &mut [c_double], w: &mut [c_double]){
	assert_eq!(J%2, 0);
	assert_eq!(J+1, b.len());
	assert_eq!(J+1, w.len());

	// k = m + J/2
	for (k, item) in b.iter_mut().enumerate() {
		*item = 2.0 * fe * sinc(2.0 * PI * fe * (k as f64 - (J/2) as f64));
	}

	for (m, item) in b.iter_mut().enumerate(){
		*item *= w[m];
	}

}

#[allow(non_snake_case)]
pub fn safe_FIR_filtering(x: &[c_double], y: &mut [c_double], L: usize, b: &mut [c_double], J: usize){
	// check index here
	assert_eq!(J+1, b.len());

	for n in 0..L {
		for m in 0..=J {
			if n >= m {
				y[n] += b[m] * x[n-m];
			}
		}
	}
}


#[link(name = "iir_filter")]
extern {
	 /*pub*/ //fn IIR_LPF(fc  : c_double, Q:  c_double , a: *mut c_double, b: *mut c_double)
	pub fn IIR_HPF(fc  : c_double, Q:  c_double , a: *mut c_double, b: *mut c_double)
	;pub fn IIR_BPF(fc1 : c_double, fc2: c_double, a: *mut c_double, b: *mut c_double)
	;pub fn IIR_BEF(fc1 : c_double, fc2: c_double, a: *mut c_double, b: *mut c_double)
	;/*pub*/ //fn IIR_resonator(fc: c_double, Q: c_double, a: *mut c_double, b: *mut c_double)
	pub fn IIR_notch(fc: c_double, Q: c_double, a: *mut c_double, b: *mut c_double)
	;pub fn IIR_low_shelving(fc: c_double, Q: c_double,g: c_double, a: *mut c_double, b: *mut c_double)
	;pub fn IIR_high_shelving(fc: c_double, Q: c_double,g: c_double, a: *mut c_double, b: *mut c_double)
	;pub fn IIR_peaking(fc: c_double, Q: c_double,g: c_double, a: *mut c_double, b: *mut c_double)
	;//*pub*/ fn IIR_filtering(x: *const c_double, y: *mut c_double, L: c_int, a: *const c_double, b: *const c_double, I: c_int, J: c_int);
}


#[allow(non_snake_case)]
pub fn safe_IIR_LPF(fc  : c_double, Q:  c_double , a: &mut [c_double], b: &mut [c_double]){
  	let fc = (PI * fc).tan() / (2.0 * PI);

    a[0] = 1.0 + 2.0 * PI * fc / Q + 4.0 * PI * PI * fc * fc;
    a[1] = (8.0 * PI * PI * fc * fc - 2.0) / a[0];
    a[2] = (1.0 - 2.0 * PI * fc / Q + 4.0 * PI * PI * fc * fc) / a[0];
    b[0] = 4.0 * PI * PI * fc * fc / a[0];
    b[1] = 8.0 * PI * PI * fc * fc / a[0];
    b[2] = 4.0 * PI * PI * fc * fc / a[0];
    
    a[0] = 1.0;
}

#[allow(non_snake_case)]
pub fn safe_IIR_filtering(x: &[c_double], y: &mut [c_double], L: usize, a: &[c_double], b: &[c_double], I: usize, J: usize){
	assert_eq!(J+1, b.len());
	assert_eq!(I+1, a.len());
	assert_eq!(L, x.len());
	assert_eq!(L, y.len());

	for n in 0..L {
		for m in 0..=J {
			if n >= m {
				y[n] += b[m] * x[n-m];
			}
		}
		for m in 1..=I {
			if n >= m {
				y[n] += -a[m] * y[n-m];
			}
		}
	}

}

#[allow(non_snake_case)]
pub fn safe_IIR_resonator(fc: c_double, Q: c_double, a: &mut [c_double], b: &mut [c_double]){
	assert_eq!(3, a.len());
	assert_eq!(3, b.len());
  	let fc = (PI * fc).tan() / (2.0 * PI);

    a[0] = 1.0 + 2.0 * PI * fc / Q + 4.0 * PI * PI * fc * fc;
    a[1] = (8.0 * PI * PI * fc * fc - 2.0) / a[0];
    a[2] = (1.0 - 2.0 * PI * fc / Q + 4.0 * PI * PI * fc * fc) / a[0];
    b[0] = 2.0 * PI * fc / Q / a[0];
    b[1] = 0.0;
    b[2] = -2.0 * PI * fc / Q / a[0];
    
    a[0] = 1.0;
}
