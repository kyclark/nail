use lazy_static::lazy_static;

pub trait PrintMe {
    fn print(&self);
}

impl PrintMe for String {
    fn print(&self) {
        println!("{}", self)
    }
}

impl PrintMe for usize {
    fn print(&self) {
        println!("{}", self)
    }
}

impl PrintMe for i32 {
    fn print(&self) {
        println!("{}", self)
    }
}

impl PrintMe for f32 {
    fn print(&self) {
        println!("{:.8}", self)
    }
}

pub trait LogAbuse {
    fn ln_or_inf(self) -> f32;
}

impl LogAbuse for f32 {
    fn ln_or_inf(self) -> f32 {
        return if self == 0.0 {
            -f32::INFINITY
        } else {
            self.ln()
        };
    }
}

pub fn f32_vec_argmax(vec: &Vec<f32>) -> usize {
    let mut max: f32 = vec[0];
    let mut argmax: usize = 0;

    for i in 1..vec.len() {
        if vec[i] > max {
            max = vec[i];
            argmax = i;
        }
    }
    argmax
}

pub fn max_of_f32_matrix(matrix: &Vec<Vec<f32>>) -> f32 {
    let mut max = 0.0;
    for row in matrix {
        for value in row {
            let abs_value = value.abs();
            if abs_value != f32::INFINITY && abs_value >= max {
                max = abs_value
            }
        }
    }
    max
}

lazy_static! {
    pub static ref LOGSUM_LOOKUP: Vec<f32> = {
        let mut f: Vec<f32> = vec![];
        for i in 0..LOGSUM_TABLE_SIZE {
            f.push((1.0 + (-(i as f64) / LOGSUM_SCALE as f64).exp()).ln() as f32);
        }
        f
    };
}

const LOGSUM_SCALE: f32 = 1000.0;
const LOGSUM_TABLE_SIZE: usize = 16000;

pub fn log_sum_table_dump() {
    let mut line_break_counter = 0;
    for i in 0..LOGSUM_TABLE_SIZE {
        line_break_counter += 1;
        print!("{:2.8} ", LOGSUM_LOOKUP[i]);
        if line_break_counter > 8 {
            println!();
            line_break_counter = 0;
        }
    }
}

/// A fast, table driven approximation of the sum of two floats in log space.
#[inline(always)]
pub fn log_sum(a: f32, b: f32) -> f32 {
    let min = f32::min(a, b);
    let max = f32::max(a, b);

    // TODO: we probably don't want these guards here for release
    //       but maybe we'll use scaling instead of log space, so it might not matter
    if a.is_nan() {
        panic!("a is nan in log_sum");
    } else if a == f32::INFINITY {
        panic!("a is +inf in log_sum");
    } else if b.is_nan() {
        panic!("b is nan in log_sum");
    } else if b == f32::INFINITY {
        panic!("b is +inf in log_sum");
    }

    return if min == -f32::INFINITY || max - min >= 15.7 {
        max
    } else {
        max + LOGSUM_LOOKUP[((max - min) * LOGSUM_SCALE) as usize]
    };
}