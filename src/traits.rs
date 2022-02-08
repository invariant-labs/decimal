pub trait OpsUp<T> {
    fn mul_up(&self, other: &T) -> T;
    fn div_up(&self, other: &T) -> T;
}

pub trait BigOps<T> {
    fn big_mul(&self, other: &T) -> T;
    fn big_mul_up(&self, other: &T) -> T;
    fn big_div(&self, other: &T) -> T;
    fn big_div_up(&self, other: &T) -> T;
}
