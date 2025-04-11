use clipboard::{ClipboardContext, ClipboardProvider};

pub fn copy_to_clipboard(text: &str) -> Result<(), String> {
    let mut ctx: ClipboardContext = ClipboardProvider::new()
        .map_err(|e| format!("Failed to initialize clipboard: {}", e))?;
    
    ctx.set_contents(text.to_string())
        .map_err(|e| format!("Failed to copy to clipboard: {}", e))
}