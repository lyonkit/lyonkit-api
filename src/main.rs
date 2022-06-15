use std::process::ExitCode;

fn main() -> ExitCode {
  println!("Hello ! This is the root project binary. I am useless, run with flag '-p server' to run the server");
  ExitCode::FAILURE
}
