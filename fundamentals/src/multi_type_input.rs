fn main() {
    let a: u8 = 3;
    let b: u16 = 32;

    let result_into = add_into(a, b);
    let result_from = add_from(a, b);
    println!("Sum into: {}", result_into);
    println!("Sum from: {}", result_from);
}

// Functions can take any value that can be turned into `f64`

fn add_into<TA, TB>(a: TA, b: TB) -> f64
where
    TA: Into<f64>,
    TB: Into<f64>,
{
    a.into() + b.into()
}


fn add_from<TA, TB>(a: TA, b: TB) -> f64
where
    f64: From<TA>,
    f64: From<TB>
{
    f64::from(a) + f64::from(b)
}
