use std::fmt::Display;

#[derive(Debug)]
pub struct Nodex {
    pub values: Vec<i32>,
    pub children: Vec<Nodex>,
}

pub struct NodeIter<'a> {
    pub viter: Box<dyn Iterator<Item = &'a i32> + 'a>,
    pub citer: Box<dyn Iterator<Item = &'a Nodex> + 'a>,
}

impl<'a> Iterator for NodeIter<'a> {
    type Item = &'a i32;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(val) = self.viter.next() {
            Some(val)
        } else {
            if let Some(child) = self.citer.next() {
                self.viter = Box::new(child.values.iter());
                self.next()
            } else {
                None
            }
        }
    }
}

impl Nodex {
    pub fn vals<'a>(&'a self) -> Box<dyn Iterator<Item = &i32> + 'a> {
        Box::new(
            self.values
                .iter()
                .chain(self.children.iter().map(|x| x.vals()).flatten()),
        )
    }
}
