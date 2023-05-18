#[derive(Default)]
pub struct AsciiAnimation<'a> {
    width: u8,
    height: u8,
    frames: Vec<&'a str>,
    buffered_frames: Vec<String>,
    index: usize,
    backward: bool,
}

impl<'a> AsciiAnimation<'a> {
    pub fn new(backing_buffer: &'a str) -> Self {
        AsciiAnimation {
            frames: backing_buffer.split(">\n").collect(),
            ..Default::default()
        }
    }

    pub fn next_frame(&mut self) -> &str {
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

        self.buffered_frames
            .get(self.index)
            .map(|buf_frames| buf_frames.as_ref())
            .unwrap_or(self.frames[self.index])
    }

    fn update_buffered_frame(&mut self) {
        let pad_height = ((self.height as i32 - self.frames[0].split('\n').count() as i32) / 2)
            .clamp(0, 1024) as usize;

        self.buffered_frames = self
            .frames
            .iter()
            .map(|frame| {
                let frame = frame
                    .split('\n')
                    .map(|line| {
                        let pad_width = ((self.width as i32 - line.chars().count() as i32) / 2)
                            .clamp(0, 1024) as usize;
                        format!("\x1b[{}C", pad_width) + &line
                    })
                    .collect::<Vec<String>>()
                    .join("\n");
                format!("\x1b[{}B{}", pad_height, frame)
            })
            .collect();
    }

    pub fn set_width(&mut self, width: u8) {
        if self.width != width {
            self.width = width;
            self.update_buffered_frame();
        }
    }

    pub fn set_height(&mut self, height: u8) {
        if self.height != height {
            self.height = height;
            self.update_buffered_frame();
        }
    }
}
