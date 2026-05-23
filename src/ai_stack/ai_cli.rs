// ============================================================================
// MODULE: ai_client.rs
// PURPOSE: Handles the direct HTTP communication with the Gemini API.
// ============================================================================

// STEP 1: Import necessary dependencies
// - Bring `std::env` into scope for environment variable fetching.
// - Import the `json!` macro from `serde_json` for dynamic payload construction.
// - Import `reqwest` to manage the asynchronous HTTP client and requests.

// STEP 2: Define the asynchronous function signature
// - The function should be `pub async fn send_to_gemini(prompt: &str)`
// - It must return a `Result<String, Box<dyn std::error::Error>>` to allow flexible error handling.

    // STEP 3: Fetch and validate the API Key from the environment
    // - Retrieve the "GEMINI_API_KEY" variable.
    // - Use `.map_err()` to intercept a missing variable error and convert it into 
    //   a clear, user-friendly error message string.
    // - Apply the `?` operator to return early if the key is missing.

    // STEP 4: Prepare the request payload
    // - Construct the final destination URL using `format!` by injecting the API key into Google's base endpoint.
    // - Use the `json!` macro to model the strict JSON structure required by Google:
    //   { "contents": [{ "parts": [{ "text": prompt }] }] }

    // STEP 5: Dispatch the HTTP Request and parse the response asynchronously
    // - Instantiate a new HTTP client via `reqwest::Client::new()`.
    // - Build a POST request to the URL, attach the JSON body, and fire it using `.send().await?`.
    // - Check if the server responded with a success status code (`is_success()`):
    //   - IF SUCCESSFUL: 
    //     * Parse the raw body into a dynamic `serde_json::Value` using `.json().await?`.
    //     * Safely navigate down the JSON tree structure to extract the raw text slice:
    //       `["candidates"][0]["content"]["parts"][0]["text"]`
    //     * Use `if let Some(text) = ...as_str()` to verify it's a valid string.
    //     * Convert the slice to an owned `String` and return it wrapped in `Ok(...)`.
    //   - IF IT FAILS (or JSON path is missing):
    //     * Format a descriptive fallback error message including the HTTP status, 
    //       allocate it on the Heap using `.into()`, and return an `Err(...)`.

    use std::env;
    use serde_json::json;
    use reqwest;

    pub async fn send_to_gemini(prompt: &str) -> Result<String, Box<dyn std::error::Error>>
    {
        // Fetch and validate the API Key from the environment
        let api_key = env::var("GEMINI_API_KEY").map_err(|_| "API not found")?;

        // Prepare the request payload 
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-3.5-flash:generateContent?key={}",
            api_key
        );
        
        let body = json!({ 
            "contents": [{ "parts": [{ "text": prompt }] }] 
        });

        // Instantiate a new HTTP client
        let client: reqwest::Client = reqwest::Client::new();

        // Build a POST request to the URL
        let response = client.post(url).json(&body).send().await?;

        let status = response.status();
        if status.is_success() {
            let res_json : serde_json::Value = response.json().await?;
            if let Some(text) = res_json["candidates"][0]["content"]["parts"][0]["text"].as_str(){
                return Ok(text.to_string());
            }
        }
        Err(format!("Gemini not response, status{}",status).into())

    }