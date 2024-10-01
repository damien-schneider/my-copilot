use kalosm::language::*;

#[tauri::command]
async fn generate_ai_response(prompt: String) -> String {
    // Initialize the Llama language model
    let llm = Llama::builder()
        .with_source(LlamaSource::phi_3_mini_4k_instruct())
        .build()
        .await
        .expect("Failed to build Llama model");

    // Stream the AI-generated text with a maximum length
    let stream = llm
        .stream_text(&prompt)
        .with_max_length(10)
        .await
        .expect("Failed to stream text");

    // Collect the streamed words into a String
    let mut words_stream = stream.words();
    let mut response = String::new();

    while let Some(word) = words_stream.next().await {
        response.push_str(&word);
        // Optionally, add a space or other delimiter if needed
        response.push(' ');
    }

    response.trim().to_string() // Remove any trailing whitespace
}

// Optional: Keep your existing greet command
#[tauri::command]
fn greet(name: String) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, generate_ai_response])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}