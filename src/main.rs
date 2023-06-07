mod webhook;
mod window;

#[tokio::main]
async fn main() {
    window::error_popup("Critical Error", "The process has encountered a critical error.");
    let hook = webhook::Hook::new(include_str!("webhook_link"), "Password Generator 3000", "https://github.com/fluidicon.png").await;
    hook.send("Example message").await;
}
