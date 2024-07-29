pub struct TmpPoem {
    pub nn: u32,
    pub lines: Vec<String>,
}

impl TmpPoem {
    pub fn new(nn: u32) -> Self {
        Self {
            nn,
            lines: vec![],
        }
    }

    pub fn add(&mut self, line: String) {
        self.lines.push(line)
    }
}