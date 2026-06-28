// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri_plugin_shell::ShellExt;

#[tauri::command]
async fn download_single(
    app_handle: tauri::AppHandle,
    url: String,
    media_format: String,
    output_path: String
) -> Result<String, String> {

    // 1. Initialize the yt-dlp sidecar
    let mut cmd = app_handle
        .shell()
        .sidecar("yt-dlp")
        .map_err(|e| format!("Failed to boot sidecar: {}", e))?;

    // 2. Apply baseline arguments (warnings off, target output path)
    cmd = cmd
        .arg("--no-warnings")
        .arg("--output").arg(&output_path);

    // 3. Basic format routing based on your Vue variable
    if media_format == "mp3" {
        cmd = cmd
            .arg("--extract-audio")
            .arg("--audio-format").arg("mp3")
            .arg("--audio-quality").arg("0");
    } else if media_format == "mp4" {
        cmd = cmd
            .arg("--format").arg("bestvideo[ext=mp4]+bestaudio[ext=m4a]/best[ext=mp4]/best")
            .arg("--merge-output-format").arg("mp4");
    } else if media_format == "wav" {
         cmd = cmd
            .arg("--extract-audio")
            .arg("--audio-format").arg("wav")
            .arg("--audio-quality").arg("0");
    } else {
        cmd = cmd.arg("--format").arg("bestvideo+bestaudio/best");
    }


    cmd = cmd.arg(url);

    let output = cmd.output().await.map_err(|e| e.to_string())?;


    if output.status.success() {
        Ok(String::from("BD klepped successfully."))
    } else {
        let err_msg = String::from_utf8_lossy(&output.stderr).into_owned();
        Err(format!("{}", err_msg))
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![download_single])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
