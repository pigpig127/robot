use anyhow::Result;
use log::info;
use tauri::{updater::UpdateResponse, AppHandle, Manager, Wry};

pub fn run_check_update(app: AppHandle<Wry>, silent: bool, has_msg: Option<bool>) {
    tauri::async_runtime::spawn(async move {
        if let Ok(update_resp) = app.updater().check().await {
            if update_resp.is_update_available() {
                if silent {
                    tauri::async_runtime::spawn(async move {
                        silent_install(app, update_resp).await.unwrap();
                    });
                } else {
                    tauri::async_runtime::spawn(async move {
                        prompt_for_install(app, update_resp).await.unwrap();
                    });
                }
            } else if let Some(v) = has_msg {
                if v {
                    let windows = app.windows();
                    let parent_window = windows.values().next();
                    tauri::api::dialog::message(
                        parent_window,
                        "ChatGPT",
                        "Your ChatGPT is up to date",
                    );
                }
            }
        }
    });
}

pub async fn silent_install(app: AppHandle<Wry>, update: UpdateResponse<Wry>) -> Result<()> {
    info!("silent_install");
    let windows = app.windows();
    let parent_window = windows.values().next();

    // Launch updater download process
    // macOS we display the `Ready to restart dialog` asking to restart
    // Windows is closing the current App and launch the downloaded MSI when ready (the process stop here)
    // Linux we replace the AppImage by launching a new install, it start a new AppImage instance, so we're closing the previous. (the process stop here)
    update.download_and_install().await?;

    // Ask user if we need to restart the application
    let should_exit = tauri::api::dialog::blocking::ask(
        parent_window,
        "Ready to Restart",
        "The silent installation was successful, do you want to restart the application now?",
    );
    if should_exit {
        app.restart();
    }

    Ok(())
}

pub async fn prompt_for_install(app: AppHandle<Wry>, update: UpdateResponse<Wry>) -> Result<()> {
    info!("prompt_for_install");
    let windows = app.windows();
    let parent_window = windows.values().next();
    let package_info = app.package_info().clone();

    let body = update.body().unwrap();
    // todo(lemarier): We should review this and make sure we have
    // something more conventional.
    let should_install = tauri::api::dialog::blocking::ask(
        parent_window,
        format!(r#"A new version of {} is available! "#, package_info.name),
        format!(
            r#"{} {} is now available -- you have {}.

Would you like to install it now?

Release Notes:
{}"#,
            package_info.name,
            update.latest_version(),
            package_info.version,
            body
        ),
    );

    if should_install {
        // Launch updater download process
        // macOS we display the `Ready to restart dialog` asking to restart
        // Windows is closing the current App and launch the downloaded MSI when ready (the process stop here)
        // Linux we replace the AppImage by launching a new install, it start a new AppImage instance, so we're closing the previous. (the process stop here)
        update.download_and_install().await?;

        // Ask user if we need to restart the application
        let should_exit = tauri::api::dialog::blocking::ask(
            parent_window,
            "Ready to Restart",
            "The installation was successful, do you want to restart the application now?",
        );
        if should_exit {
            app.restart();
        }
    }

    Ok(())
}
