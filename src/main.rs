use std::collections::HashMap;
use std::process;

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
      // FIXME: format! only supports string literal.
      Some(&arg_str) => format!(String::from(arg_str), args),
      _ => None,
    }
  }
}

fn run() {
  for cmd in build_cmd {
    let output = process::Command::new(cmd.name)
      .args(cmd.args)
      .output()
      .expect(&format!("{} failed", &cmd.name));
    if !output.status.success() {
      println!("{}", String::from_utf8_lossy(&output.stderr));
      if !cmd.ignore {
        break;
      }
    }
  }
}

fn main() {
  run();
}
