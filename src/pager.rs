pub struct Pager {
    pub start: i32,
    pub count: i32,
    pub total_count: i32,
}

impl Pager {
    pub fn new() -> Pager {
        Pager {
            start: 0,
            count: 20,
            total_count: 20,
        }
    }

    fn reset(&mut self) {
        self.start = 0;
        self.count = if self.total_count > 40 { 100 } else { 20 }
    }

    pub fn set_total_count(&mut self, total_count: i32) {
        self.total_count = total_count;
        let remaining = total_count - self.start - self.count;
        self.count = if remaining > 40 { 100 } else { 20 }
    }

    fn next_page(&mut self) {
        self.start += self.count;
    }
}

impl Default for Pager {
    fn default() -> Self {
        Pager::new()
    }
}

impl Iterator for Pager {
    type Item = bool;


    fn next(&mut self) -> Option<bool> {
        self.next_page();

        let next = self.start < self.total_count;

        if !next {
            self.reset();
        }

        Some(next)
    }
}
