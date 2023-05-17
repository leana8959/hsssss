#[derive(Default)]
pub struct AsciiAnimation<'a> {
    height: usize,
    width: usize,
    frames: Vec<&'a str>,
    index: usize,
    backward: bool,
}

impl<'a> AsciiAnimation<'a> {
    pub fn new(backing_buffer: &'a String) -> Self {
        let frames = backing_buffer.split(">\n").collect::<Vec<&str>>();
        AsciiAnimation {
            frames,
            ..Default::default()
        }
    }

    pub fn next_frame(&mut self) -> String {
        if !self.backward {
            if self.index < self.frames.len() - 1 {
                self.index += 1;
            } else {
                self.index -= 1;
                self.backward = true;
            }
        } else {
            if self.index > 0 + 1 {
                self.index -= 1;
            } else {
                self.index += 1;
                self.backward = false;
            }
        }

        self.frames[self.index]
            .split('\n')
            .map(|line| " ".repeat(self.width / 2).to_string() + &line)
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn set_width(&mut self, width: usize) {
        self.width = width;
    }

    pub fn set_height(&mut self, width: usize) {
        unimplemented!();
    }
}
