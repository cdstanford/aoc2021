use aoc2021::util;

/*
    Iterator over adjacent pairs
*/
struct PairIter<T, I: Iterator<Item = T>> {
    prev: Option<T>,
    sub: I,
}
impl<T, I> Iterator for PairIter<T, I>
where
    T: Copy,
    I: Iterator<Item = T>,
{
    type Item = (T, T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.prev.is_none() {
            self.prev = Some(self.sub.next()?);
        }
        let t1 = self.prev.unwrap();
        let t2 = self.sub.next()?;
        self.prev = Some(t2);
        Some((t1, t2))
    }
}

fn adjacencies<T, I>(iter: I) -> impl Iterator<Item = (T, T)>
where
    T: Copy,
    I: Iterator<Item = T>,
{
    PairIter { prev: None, sub: iter }
}

fn main() {
    let depths: Vec<usize> = util::file_to_vec_parsed("input/day01.txt");
    let result = adjacencies(depths.iter()).filter(|(d1, d2)| d1 < d2).count();
    println!("Part 1 answer: {}", result);
}
