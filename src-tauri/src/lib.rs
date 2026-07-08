// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri_plugin_shell::ShellExt;

#[tauri::command]
async fn download_single(
    app_handle: tauri::AppHandle,
    url: String,
    media_format: String,
    output_path: String
) -> Result<String, String> {

    let mut cmd = app_handle
        .shell()
        .sidecar("yt-dlp")
        .map_err(|e| format!("Failed to boot sidecar: {}", e))?;

    cmd = cmd
        .arg("--no-warnings")
        .arg("--output").arg(output_path);

    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            cmd = cmd.arg("--ffmpeg-location").arg(exe_dir);
        }
    }

    if media_format == "mp3" {
        cmd = cmd
            .arg("--extract-audio")
            .arg("--audio-format").arg("mp3")
            .arg("--audio-quality").arg("0");
    }
    else if media_format == "mp4" {
        cmd = cmd
            .arg("--format").arg("bestvideo+bestaudio/best")
            .arg("--use-postprocessor").arg("FFmpegCopyStream")
            .arg("--ppa").arg("CopyStream:-c:v libx264 -c:a aac");
    }
    else if media_format == "wav" {
         cmd = cmd
            .arg("--extract-audio")
            .arg("--audio-format").arg("wav")
            .arg("--audio-quality").arg("0");
    }
    else if media_format == "ogg" {
         cmd = cmd
            .arg("--extract-audio")
            .arg("--audio-format").arg("vorbis")
            .arg("--audio-quality").arg("0");
    }
    else {
        cmd = cmd
            .arg("--format").arg("bestvideo[vcodec^=avc]+bestaudio[acodec^=mp4a]/best")
            .arg("--recode-video").arg("mp4");
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

#[tauri::command]
async fn check_sidecars(app: tauri::AppHandle) -> Result<bool, String> {
    let ytdlp_ok = match app.shell().sidecar("yt-dlp") {
        Ok(cmd) => cmd.arg("--version").output().await.is_ok(),
        Err(_) => false,
    };

    let ffmpeg_ok = match app.shell().sidecar("ffmpeg") {
        Ok(cmd) => cmd.arg("-version").output().await.is_ok(),
        Err(_) => false,
    };

    Ok(ytdlp_ok && ffmpeg_ok)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![download_single, check_sidecars])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
