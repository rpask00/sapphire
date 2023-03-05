pub static TAX_MULTIPLIER: f64 = 0.0115;


pub enum Currency {
    PLN = 6
}

impl From<Currency> for i32 {
    fn from(c: Currency) -> i32 {
        c as i32
    }
}
