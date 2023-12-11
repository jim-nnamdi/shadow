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

pub struct Nodef {
    pub value: Vec<i32>,
    pub left: Vec<Nodef>,
    pub right: Vec<Nodef>,
}

pub struct NodefIterator<'a> {
    pub nodefv: Box<dyn Iterator<Item = &'a i32> + 'a>,
    pub nodefl: Box<dyn Iterator<Item = &'a Nodef> + 'a>,
    pub nodefr: Box<dyn Iterator<Item = &'a Nodef> + 'a>,
}

impl<'a> Iterator for NodefIterator<'a> {
    type Item = &'a i32;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(nfv) = self.nodefv.next() {
            Some(nfv)
        } else {
            if let Some(nfl) = self.nodefl.next() {
                self.nodefv = Box::new(nfl.value.iter());
                self.next()
            } else {
                if let Some(nfr) = self.nodefr.next() {
                    self.nodefv = Box::new(nfr.value.iter());
                    self.next()
                } else {
                    None
                }
            }
        }
    }
}

impl Nodef {
    pub fn znodefs<'a>(&'a self) -> Box<dyn Iterator<Item = &i32> + 'a> {
        Box::new(
            self.value.iter().chain(
                self.left
                    .iter()
                    .chain(self.right.iter())
                    .map(|f| f.znodefs())
                    .flatten(),
            ),
        )
    }
}
