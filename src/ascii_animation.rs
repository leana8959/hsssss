use smallvec::SmallVec;

#[derive(Default)]
pub struct AsciiAnimation<'a> {
    height: u8,
    width: u8,
    frames: SmallVec<[&'a str; 1024]>,
    index: usize,
    backward: bool,
}

impl<'a> AsciiAnimation<'a> {
    pub fn new(backing_buffer: &'a String) -> Self {
        AsciiAnimation {
            frames: backing_buffer.split(">\n").collect(),
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
            .map(|line| {
                let pad_width =
                    ((self.width as i32 - line.chars().count() as i32) / 2).clamp(0, 1024) as usize;
                " ".repeat(pad_width).to_string() + &line
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn set_width(&mut self, width: u8) {
        self.width = width;
    }

    pub fn set_height(&mut self, height: usize) {
        unimplemented!();
    }
}
