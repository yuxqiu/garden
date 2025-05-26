pub struct Conway {
    buffer: Vec<bool>,
    width: usize,
    height: usize,
}

impl Conway {
    pub fn new(width: u16, height: u16) -> Self {
        // the game will be weird if height or width is < 3
        let width = width as usize;
        let height = height as usize;

        Self {
            buffer: vec![false; height * width],
            width: width,
            height: height,
        }
    }

    pub fn set(&mut self, i: u16, j: u16) {
        let i = i as usize;
        let j = j as usize;

        if i >= self.height || j >= self.width {
            return;
        }

        self.buffer[i * self.width + j] = true;
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        let width = width as usize;
        let height = height as usize;

        self.buffer = vec![false; height * width];
        self.height = height;
        self.width = width;
    }

    pub fn next(&mut self) {
        let mut new_state = vec![false; self.height * self.width];

        // A not efficient way to update buffer
        for i in 0..self.height {
            for j in 0..self.width {
                let sum: u8 = self
                    .neighbor(i, j)
                    .map(|(i, j)| self.get(i, j) as u8)
                    .iter()
                    .sum();
                if self.get(i, j) {
                    if sum == 2 || sum == 3 {
                        new_state[i * self.width + j] = true;
                    }
                } else {
                    if sum == 3 {
                        new_state[i * self.width + j] = true;
                    }
                }
            }
        }

        self.buffer = new_state;
    }

    pub fn state(&self) -> &[bool] {
        &self.buffer
    }

    pub fn width(&self) -> u16 {
        self.width as u16
    }

    pub fn height(&self) -> u16 {
        self.height as u16
    }

    fn neighbor(&self, i: usize, j: usize) -> [(usize, usize); 8] {
        let up = if i == 0 { self.height - 1 } else { i - 1 };
        let down = if i == self.height - 1 { 0 } else { i + 1 };
        let left = if j == 0 { self.width - 1 } else { j - 1 };
        let right = if j == self.width - 1 { 0 } else { j + 1 };

        [
            (up, left),
            (up, j),
            (up, right),
            (i, left),
            (i, right),
            (down, left),
            (down, j),
            (down, right),
        ]
    }

    fn get(&self, i: usize, j: usize) -> bool {
        self.buffer[i * self.width + j]
    }
}
