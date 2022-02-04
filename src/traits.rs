pub trait OpsUp<T> {
    fn mul_up(self, other: T) -> T;
    fn div_up(self, other: T) -> T;
}
