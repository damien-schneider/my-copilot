use once_cell::sync::Lazy;
use tokio::sync::{Mutex, Notify};
use std::sync::Arc;

use kalosm::language::*;

static LLM: Lazy<Mutex<Option<Llama>>> = Lazy::new(|| Mutex::new(None));
// A Notify instance to signal when LLM initialization is complete
static LLM_INITIALIZED: Lazy<Arc<Notify>> = Lazy::new(|| Arc::new(Notify::new()));

async fn initialize_llm() {
    let llm = Llama::builder()
        .with_source(LlamaSource::phi_3_mini_4k_instruct())
        .build()
        .await
        .expect("Failed to build Llama model");

    *LLM.lock().await = Some(llm);

    // Notify all waiting tasks that LLM has been initialized
    LLM_INITIALIZED.notify_waiters();
}

#[tauri::command]
async fn generate_ai_response(prompt: String) -> String {
    // Wait until the LLM is initialized
    {
        let llm_lock = LLM.lock().await;
        if llm_lock.is_none() {
            // Release the lock before waiting
            drop(llm_lock);
            // Wait for the LLM to be initialized
            LLM_INITIALIZED.notified().await;
        }
    }

    // Acquire the lock again after ensuring initialization
    let llm_lock = LLM.lock().await;

    if let Some(ref llm) = *llm_lock {
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
    } else {
        "LLM not initialized".to_string()
    }
}

// Optional: Keep your existing greet command
#[tauri::command]
fn greet(name: String) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|_app| {
            // Spawn the LLM initialization without blocking
            tauri::async_runtime::spawn(async {
                initialize_llm().await;
            });
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, generate_ai_response])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}