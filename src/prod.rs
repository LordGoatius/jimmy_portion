pub struct CartesianProduct {
    max: usize,
    color: usize,
    size: usize,
    num: usize,
}

impl CartesianProduct {
    pub fn new(color: usize, size: usize) -> Self {
        let max = usize::pow(color, size as u32);
        CartesianProduct { max, color, size, num: 0usize }
    }
}

impl Iterator for CartesianProduct {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.max == self.num {
            None
        } else {
            let mut to_ret: Vec<usize> = vec![];

            for i in 0usize..self.size {
                to_ret.push((self.num / usize::pow(self.color, i as u32)) % self.color);
            }

            self.num += 1;

            Some(to_ret)
        }
    }
}
