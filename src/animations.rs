const SPINNER_FRAMES: &[&'static str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

pub struct Animation{
  frames: &'static[&'static str],
  current_frame: usize,
  render_count: usize,
  buffer: usize
}

pub enum AnimationType{
  LOADING
}

impl Animation{
  pub fn new(ani_type: AnimationType, buffer: Option<usize>) -> Animation{
    let buffer = buffer.unwrap_or(1);
    match ani_type {
        AnimationType::LOADING => Animation{
          current_frame: 0,
          frames: SPINNER_FRAMES,
          render_count: 0,
          buffer: buffer
        }
    }
  }

  #[allow(dead_code)]
  pub fn curr_frame(&self) -> String{
    self.frames[self.current_frame].to_owned()
  }

  pub fn next_frame(&mut self) -> String {
    self.render_count = (self.render_count + 1) % self.buffer;
    if (self.render_count % self.buffer) != 0 {
      return self.curr_frame();
    }
    self.current_frame = (self.current_frame + 1) % SPINNER_FRAMES.len();
    self.frames[self.current_frame].to_owned()
  }
}
