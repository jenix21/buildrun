extern crate regex;
use std::collections::HashMap;
use std::process;

trait Formattable {
  fn format(&self, args: &[&str]) -> Self;
}

impl Formattable for String {
  fn format(&self, args: &[&str]) -> String {
    let place_holder = "{}";
    if !self.contains(&place_holder) {
      return self.clone();
    }
    let mut formatted = String::new();
    let slices: Vec<&str> = self.split("{}").collect();
    for zipped in slices.iter().zip(args.iter()) {
      formatted.push_str(zipped.0);
      formatted.push_str(zipped.1);
    }
    return formatted;
  }
}

struct BuildCommand {
  cmd_map: HashMap<&'static str, &'static str>,
}

impl BuildCommand {
  fn new() -> Self {
    BuildCommand {
      cmd_map: HashMap::new(),
    }
  }
  fn init(&mut self) {
    self.cmd_map.insert("gclient", "sync --verbose --reset --force --with_branch_heads --ignore_locks --delete_unversioned_trees --disable-syntax-validation");
    self.cmd_map.insert("gn", "gen {}");
    self
      .cmd_map
      .insert("incredibuild.cmd", "{} {}");
  }
  fn args_of(&self, cmd: &str, args: &[&str]) -> Option<String> {
    match self.cmd_map.get(cmd) {
      Some(ref arg_str) => Some(arg_str.to_string().format(args)),
      _ => None,
    }
  }
}

fn run() {
  let mut build_cmd = BuildCommand::new();
  build_cmd.init();

  let cmd_to_run = vec![("gn", "out/release"), ("incredibuild.com", "release,whale")];

  for cmd_arg in &cmd_to_run {
    let args: Vec<&str> = cmd_arg.1.split(',').collect();
    let output = process::Command::new(cmd_arg.0)
      .args(build_cmd.args_of(cmd_arg.0, &args))
      .output()
      .expect(&format!("{} failed", &cmd_arg.0));
    if !output.status.success() {
      println!("{}", String::from_utf8_lossy(&output.stderr));
    }
  }
}

fn main() {
  run();
}
