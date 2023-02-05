use std::process::Command;
use std::env;

fn main() {
    glib_build_tools::compile_resources(
        "src/resources",
        "src/resources/resources.gresource.xml",
        "chatgpt.gresource",
    );

    let home_dir = env::var("HOME").unwrap();
    let mut schemas_dir = home_dir;
    schemas_dir.push_str("/.local/share/glib-2.0/schemas");
    Command::new("mkdir").args(&["-p", &schemas_dir])
        .status().unwrap();
    schemas_dir.push_str("/");
    Command::new("cp").args(&["src/org.teunissenstefan.ChatGPT.gschema.xml", &schemas_dir])
        .status().unwrap();
    Command::new("glib-compile-schemas").args(&[&schemas_dir])
        .status().unwrap();

    println!("cargo:rerun-if-changed=src/resources/resources.gresource.xml");
    println!("cargo:rerun-if-changed=src/org.teunissenstefan.ChatGPT.gschema.xml");
}