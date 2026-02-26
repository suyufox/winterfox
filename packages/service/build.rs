
const COMMANDS: &[&str] = &[
  "ping",
  "warn",
  "debug",
  "info",
  "error",
  "trace"
];

fn main() {
  tauri_plugin::Builder::new(COMMANDS)
    .android_path("android")
    .ios_path("ios")
    .build();
}
