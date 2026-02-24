use crate::models::ScreenshotResponse;
use crate::Result;
use tauri::Runtime;

use crate::desktop::ScreenshotContext;
use crate::shared::ScreenshotParams;

// Unix-specific implementation for taking screenshots (fallback for non-macOS Unix systems)
// Linux does not have native screenshot support via Tauri's webview API.
pub async fn take_screenshot<R: Runtime>(
    _params: ScreenshotParams,
    _window_context: ScreenshotContext<R>,
) -> Result<ScreenshotResponse> {
    // Return an explicit error instead of a misleading 1px stub image.
    Ok(ScreenshotResponse {
        data: None,
        success: false,
        error: Some("Screenshots are not supported on Linux. Use macOS or Windows for native screenshot support.".to_string()),
        file_path: None,
    })
}
