use std::env;

/// Everything the welcome screen needs to know about the current run.
pub struct WelcomeData {
    pub model: String,
    pub cwd: String,
}

impl WelcomeData {
    pub fn new(model: String) -> Self {
        Self {
            model,
            cwd: display_cwd(),
        }
    }
}

/// The welcome as plain text: the word "CANDLE" beside the candle.
const WELCOME_ART: &str = r#"
 ██████╗ █████╗ ███╗   ██╗██████╗ ██╗     ███████╗
██╔════╝██╔══██╗████╗  ██║██╔══██╗██║     ██╔════╝
██║     ███████║██╔██╗ ██║██║  ██║██║     █████╗
██║     ██╔══██║██║╚██╗██║██║  ██║██║     ██╔══╝
╚██████╗██║  ██║██║ ╚████║██████╔╝███████╗███████╗
 ╚═════╝╚═╝  ╚═╝╚═╝  ╚═══╝╚═════╝ ╚══════╝╚══════╝
"#;

/// The welcome text: the art plus a `model · cwd` footer.
pub fn welcome_content(data: &WelcomeData) -> String {
    format!("{}\n\n  {} · {}", WELCOME_ART, data.model, data.cwd)
}

fn display_cwd() -> String {
    let Ok(cwd) = env::current_dir() else {
        return String::new();
    };
    cwd.to_string_lossy().to_string()
}
