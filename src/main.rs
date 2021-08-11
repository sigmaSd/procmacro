use std::{
    env, fs,
    io::{self, Write},
    process::{self, Stdio},
};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    cargo_new_proc_macro()?;
    let mut code = String::new();
    let mut input = String::new();
    loop {
        print!("[In] ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;

        input = input.trim().to_string();

        if input.starts_with(":q") {
            break Ok(());
        } else if input == ":reset" {
            code.clear();
        } else if input.ends_with(';') {
            code.push_str(&input);
        } else {
            let wraped_code = wrap_code_in_proc_macro(&code, &input);
            write_code(&wraped_code)?;
            let output = cargo_build()?;
            println!("[Out] {}", output);
        }

        input.clear();
    }
}

fn cargo_new_proc_macro() -> Result<()> {
    process::Command::new("cargo")
        .args(&["new", "pm", "--lib"])
        .current_dir(env::temp_dir())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?
        .wait()?;
    fs::write(
        env::temp_dir().join("pm/Cargo.toml"),
        r#"
[package]
name = "pm"
version = "0.1.0"
edition = "2018"

[dependencies]

[lib]
proc_macro = true
              "#,
    )?;
    fs::write(
        env::temp_dir().join("pm/src/main.rs"),
        "
fn main() {}
pm::genrerated!();
              ",
    )?;
    Ok(())
}

fn wrap_code_in_proc_macro(code: &str, input: &str) -> String {
    format!(
        "
    use proc_macro::*;
    #[proc_macro]
     pub fn genrerated(_item: TokenStream) -> TokenStream {{
    {}
     println!(\"{{:?}}\", {{\n{}\n}});
     TokenStream::new()
     }}
    ",
        code, input
    )
}

fn write_code(input: &str) -> Result<()> {
    fs::write(env::temp_dir().join("pm/src/lib.rs"), input)?;
    Ok(())
}

fn cargo_build() -> Result<String> {
    let out = process::Command::new("cargo")
        .arg("build")
        .args(&["--color", "always"])
        .current_dir(env::temp_dir().join("pm"))
        .output()?;
    if out.stdout.is_empty() {
        Ok(String::from_utf8(out.stderr)?)
    } else {
        Ok(String::from_utf8(out.stdout)?)
    }
}
