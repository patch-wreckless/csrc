use std::{
    io::Write,
    process::{Command, Stdio},
};

pub fn select_directory<I>(dirs: I) -> std::io::Result<Option<String>>
where
    I: IntoIterator<Item = String>,
{
    let mut child = Command::new("fzf")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .args([
            "--delimiter",
            " ",
            "--with-nth",
            "1",
            "--preview",
            "echo {2}",
            "--accept-nth",
            "2",
        ])
        .spawn()?;

    let stdin = child.stdin.as_mut().unwrap();
    for dir in dirs {
        writeln!(stdin, "{}", dir)?;
    }

    let output = child.wait_with_output()?;

    if !output.status.success() {
        return Ok(None);
    }

    let selected = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if selected.is_empty() {
        Ok(None)
    } else {
        Ok(Some(selected))
    }
}
