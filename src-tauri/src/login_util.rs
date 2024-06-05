#[tauri::command]
pub fn attempt_login(username: String, email: String, password: String) {
    println!("User username: {}", username);
    println!("User email: {}", email);
    println!("User password: {}", password);
}


