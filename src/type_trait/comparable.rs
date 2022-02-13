pub trait Comparable<T> {
    fn compare(&self, var: &T) -> u32;
}