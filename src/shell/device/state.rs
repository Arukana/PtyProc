use std::io;

pub struct DeviceState {
  /// Update
  idle: Option<()>,
  /// The output of new lines
  out_text: Option<Vec<u8>>,
  /// The current character
  in_character: Option<u8>,
  /// The last line
  in_line: Option<Vec<u8>>,
  /// The last line buf
  in_line_buf: Option<Vec<u8>>,
}

impl io::Read for DeviceState {

  /// The method `read` initialize the standard output.
  fn read(&mut self, output: &mut [u8]) -> io::Result<usize> {
    let len: usize = output.len();
    let mut out: Vec<u8> = Vec::with_capacity(len);

    out.extend_from_slice(output);
    self.out_text = Some(out);
    self.idle = None;
    Ok(len)
  }
}

impl io::Write for DeviceState {

  /// The method `write` initialize the standard input.
  fn write(&mut self, input: &[u8]) -> io::Result<usize> {
    let len: usize = input.len();
    let mut inp: Vec<u8> = Vec::with_capacity(len);

    inp.extend_from_slice(input);
    self.in_line_buf = Some(inp);
    self.in_character = Some(input[0]);
    self.idle = None;
    Ok(len)
  }

  fn flush(&mut self) -> io::Result<()> {
    Ok(())
  }
}

impl Default for DeviceState {
  fn default() -> DeviceState {
    DeviceState {
      idle: Some(()),
      out_text: None,
      in_character: None,
      in_line: None,
      in_line_buf: None,
    }
  }
}
