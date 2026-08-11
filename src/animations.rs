const SPINNER_FRAMES: &[&'static str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

pub struct Animation{
  frames: &'static[&'static str],
  current_frame: usize
}

pub enum AnimationType{
  LOADING
}

impl Animation{
  pub fn new(ani_type: AnimationType) -> Animation{
    match ani_type {
        AnimationType::LOADING => Animation{
          current_frame: 0,
          frames: SPINNER_FRAMES
        }
    }
  }

  #[allow(dead_code)]
  pub fn curr_frame(&self) -> String{
    self.frames[self.current_frame].to_owned()
  }

  pub fn next_frame(&mut self) -> String {
    self.current_frame += 1;
    self.current_frame %= SPINNER_FRAMES.len();
    self.frames[self.current_frame].to_owned()
  }
}
