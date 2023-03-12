use std::process::Command;

pub fn tailwind_watcher() -> Command {
    let mut command = Command::new("tailwind");
    command.args([
        "-i",
        "./tailwind/input.css",
        "-o",
        "./output/output.css",
        "--watch",
    ]);

    command
}
