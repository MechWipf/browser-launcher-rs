use std::path::Path;
use winreg::enums::*;
use winreg::RegKey;

use crate::err::LauncherError;

static APP_ID: &str = &"BrowserLauncher";
static APP_NAME: &str = &"Browser Launcher";
static APP_DESCRIPTION: &str = &"Tool for launching the right Tool for the given URI.";

pub(crate) fn app_path() -> Result<String, LauncherError> {
    let path = std::env::current_exe()?;

    if !path.is_file() {
        return Err(LauncherError::RegistryNoFile);
    }

    if !path.is_absolute() {
        return Err(LauncherError::RegistryNotAbsolute);
    }

    match path.to_str() {
        None => Err(LauncherError::RegistryInvalid),
        Some(p) => Ok(p.to_owned()),
    }
}

fn app_icon() -> Result<String, LauncherError> {
    Ok(app_path()? + ",0")
}

pub(crate) fn register() -> Result<(), LauncherError> {
    let l_app_path = &app_path()?;
    let l_app_icon = &app_icon()?;

    // Register application
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

    // Register capabilities
    let path_capabilities = Path::new("SOFTWARE")
        .join("Clients")
        .join("StartMenuInternet")
        .join(APP_ID)
        .join("Capabilities");
    let (capability_key, _) = hklm.create_subkey(&path_capabilities)?;
    capability_key.set_value("ApplicationName", &APP_NAME)?;
    capability_key.set_value("ApplicationIcon", l_app_icon)?;
    capability_key.set_value("ApplicationDescription", &APP_DESCRIPTION)?;

    // Set-up protocolls to be handled
    let path = Path::new("URLAssociations");
    let (url_assoc_key, _) = capability_key.create_subkey(&path)?;
    url_assoc_key.set_value("http", &format!("{}URL", APP_ID))?;
    url_assoc_key.set_value("https", &format!("{}URL", APP_ID))?;
    url_assoc_key.set_value("ftp", &format!("{}URL", APP_ID))?;

    // Add to RegistredApplication to make it visible for windows
    let path = Path::new("SOFTWARE").join("RegisteredApplications");
    let register_key = hklm.open_subkey_with_flags(&path, KEY_ALL_ACCESS)?;
    register_key.set_value(APP_ID, &path_capabilities.to_str().unwrap().to_owned())?;

    // Add URL Handler
    let path = Path::new("SOFTWARE")
        .join("Classes")
        .join(&format!("{}URL", APP_ID));
    let (handler_key, _) = hklm.create_subkey(&path)?;
    handler_key.set_value("", &APP_NAME)?;
    handler_key.set_value("FriendlyTypeName", &APP_NAME)?;

    // Add open command
    let path = path.join("shell").join("open").join("command");
    let (command_key, _) = hklm.create_subkey(&path)?;
    command_key.set_value("", &format!("\"{}\" \"%1\"", l_app_path))?;

    Ok(())
}

pub(crate) fn unregister() -> Result<(), LauncherError> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

    // Delete application information
    let path = Path::new("SOFTWARE")
        .join("Clients")
        .join("StartMenuInternet")
        .join(APP_ID);
    let path_str = path.as_os_str().to_owned();
    match hklm.delete_subkey_all(path) {
        Ok(_) => (),
        Err(msg) => log::warn!(
            "{} Path: HKEY_LOCAL_MACHINE\\{}.",
            msg,
            path_str.to_str().unwrap()
        ),
    };

    // Delete application registration
    let path = Path::new("SOFTWARE").join("RegisteredApplications");
    let path_str = path.as_os_str().to_owned();
    let key = match hklm.open_subkey_with_flags(&path, KEY_ALL_ACCESS) {
        Ok(key) => Some(key),
        Err(msg) => {
            log::warn!(
                "{} Path: HKEY_LOCAL_MACHINE\\{}.",
                msg,
                path_str.to_str().unwrap()
            );
            None
        }
    };

    if let Some(key) = key {
        match key.delete_value(APP_ID) {
            Ok(_) => (),
            Err(msg) => log::warn!(
                "{} Path: HKEY_LOCAL_MACHINE\\{} value {}.",
                msg,
                path_str.to_str().unwrap(),
                APP_ID
            ),
        };
    }

    // Delete URL Handler
    let path = Path::new("SOFTWARE")
        .join("Classes")
        .join(&format!("{}URL", APP_ID));
    let path_str = path.as_os_str().to_owned();
    match hklm.delete_subkey_all(path) {
        Ok(_) => (),
        Err(msg) => log::warn!(
            "{} Path: HKEY_LOCAL_MACHINE\\{}.",
            msg,
            path_str.to_str().unwrap()
        ),
    }

    Ok(())
}