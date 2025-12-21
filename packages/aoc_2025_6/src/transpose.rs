pub trait Transpose {
    type Item;
    fn transpose(self) -> Transposed<Self::Item>
    where
        Self: Sized,
        Self::Item: Iterator;
}

pub struct Transposed<I>
where
    I: Iterator,
{
    iters: Vec<I>,
}

impl<I> Iterator for Transposed<I>
where
    I: Iterator,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut row = Vec::with_capacity(self.iters.len());

        for iter in &mut self.iters {
            match iter.next() {
                Some(item) => row.push(item),
                None => continue, // stop when any iterator ends
            }
        }

        if row.is_empty() { None } else { Some(row) }
    }
}

impl<T, I> Transpose for T
where
    T: Iterator<Item = I>,
    I: Iterator,
{
    type Item = I;

    fn transpose(self) -> Transposed<Self::Item> {
        Transposed {
            iters: self.collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_transposes() {
        let v1 = vec![1, 2, 3].into_iter();
        let v2 = vec![4, 5].into_iter();
        let v3 = vec![7, 8, 9].into_iter();

        let transposed: Transposed<_> = [v1, v2, v3].into_iter().transpose();
        let result: Vec<Vec<i32>> = transposed.collect();
        assert_eq!(result, vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 9]]);
    }
}
