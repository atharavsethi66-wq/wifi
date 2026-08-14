#[derive(serde::Serialize, serde::Deserialize)]
struct Credentials {
    username: String,
    password: String,
}

#[tauri::command]
async fn get_saved_credentials() -> Result<Option<Credentials>, String> {
    let entry = keyring::Entry::new("college_wifi_app", "credentials").map_err(|e| e.to_string())?;
    match entry.get_password() {
        Ok(secret) => {
            let creds: Credentials = serde_json::from_str(&secret).map_err(|e| e.to_string())?;
            Ok(Some(creds))
        }
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(format!("Secure storage error: {}", e)),
    }
}

#[tauri::command]
async fn save_credentials(creds: Credentials) -> Result<(), String> {
    let entry = keyring::Entry::new("college_wifi_app", "credentials").map_err(|e| e.to_string())?;
    let secret = serde_json::to_string(&creds).map_err(|e| e.to_string())?;
    entry.set_password(&secret).map_err(|e| format!("Secure storage error: {}", e))?;
    Ok(())
}

#[tauri::command]
async fn forget_credentials() -> Result<(), String> {
    let entry = keyring::Entry::new("college_wifi_app", "credentials").map_err(|e| e.to_string())?;
    match entry.delete_credential() {
        Ok(_) => Ok(()),
        Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(format!("Secure storage error: {}", e)),
    }
}

#[tauri::command]
async fn login_wifi(username: String, password: String) -> Result<String, String> {
    match wifi_connect::routes::wifi::login(&username, &password).await {
        Ok(xml) => {
            match wifi_connect::routes::wifi::check_status(&xml) {
                Some(status) => {
                    if status == "LIVE" {
                        let creds = Credentials {
                            username: username.clone(),
                            password: password.clone(),
                        };
                        match save_credentials(creds).await {
                            Ok(_) => Ok("Successfully logged in!".to_string()),
                            Err(err) => Err(format!("Logged in, but secure storage failed: {}", err)),
                        }
                    } else {
                        Err(format!("Login failed: Status is {}", status))
                    }
                }
                None => Err("Login failed: Could not determine status from server response.".to_string()),
            }
        }
        Err(e) => Err(format!("Connection error: {}", e)),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            login_wifi,
            get_saved_credentials,
            save_credentials,
            forget_credentials
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

