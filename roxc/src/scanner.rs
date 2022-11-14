pub struct Scanner<'a> {
    source: &'a str,
    done: bool,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            done: false,
        }
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            self.done = true;
            Some(self.source)
        }
    }
}
