//! HMLR System & Browser Provisioner
//! Implements universal ambient runtime detection and browser DevTools extension provisioning
//! (.NET Framework / JVM style universal ecosystem bootstrap).

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct BrowserInfo {
    pub name: &'static str,
    pub installed: bool,
    pub extension_installed: bool,
    pub profile_path: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct SystemEnvironmentReport {
    pub runtime_installed: bool,
    pub runtime_path: PathBuf,
    pub in_system_path: bool,
    pub browsers: Vec<BrowserInfo>,
}

pub struct HMLRProvisioner;

impl HMLRProvisioner {
    pub fn get_default_install_dir() -> PathBuf {
        if let Ok(user_profile) = env::var("USERPROFILE") {
            PathBuf::from(user_profile).join(".hmlr")
        } else if let Ok(home) = env::var("HOME") {
            PathBuf::from(home).join(".hmlr")
        } else {
            PathBuf::from(".hmlr")
        }
    }

    pub fn inspect_environment() -> SystemEnvironmentReport {
        let install_dir = Self::get_default_install_dir();
        let target_bin = install_dir.join("bin").join(if cfg!(windows) { "hmlr.exe" } else { "hmlr" });
        let runtime_installed = target_bin.exists();

        // Check PATH
        let in_system_path = if let Ok(path_var) = env::var("PATH") {
            let bin_dir_str = install_dir.join("bin").to_string_lossy().to_lowercase();
            path_var.to_lowercase().contains(&bin_dir_str)
        } else {
            false
        };

        let browsers = Self::detect_browsers(&install_dir);

        SystemEnvironmentReport {
            runtime_installed,
            runtime_path: target_bin,
            in_system_path,
            browsers,
        }
    }

    pub fn detect_browsers(install_dir: &Path) -> Vec<BrowserInfo> {
        let mut browsers = Vec::new();
        let ext_dir = install_dir.join("extension");

        #[cfg(target_os = "windows")]
        {
            let local_app_data = env::var("LOCALAPPDATA").unwrap_or_default();
            let app_data = env::var("APPDATA").unwrap_or_default();

            // 1. Google Chrome
            let chrome_path = PathBuf::from(&local_app_data).join(r"Google\Chrome\User Data");
            let chrome_installed = chrome_path.exists();
            browsers.push(BrowserInfo {
                name: "Google Chrome",
                installed: chrome_installed,
                extension_installed: chrome_installed && ext_dir.exists(),
                profile_path: if chrome_installed { Some(chrome_path) } else { None },
            });

            // 2. Microsoft Edge
            let edge_path = PathBuf::from(&local_app_data).join(r"Microsoft\Edge\User Data");
            let edge_installed = edge_path.exists();
            browsers.push(BrowserInfo {
                name: "Microsoft Edge",
                installed: edge_installed,
                extension_installed: edge_installed && ext_dir.exists(),
                profile_path: if edge_installed { Some(edge_path) } else { None },
            });

            // 3. Brave Browser
            let brave_path = PathBuf::from(&local_app_data).join(r"BraveSoftware\Brave-Browser\User Data");
            let brave_installed = brave_path.exists();
            browsers.push(BrowserInfo {
                name: "Brave Browser",
                installed: brave_installed,
                extension_installed: brave_installed && ext_dir.exists(),
                profile_path: if brave_installed { Some(bra_path(brave_path)) } else { None },
            });

            // 4. Mozilla Firefox
            let firefox_path = PathBuf::from(&app_data).join(r"Mozilla\Firefox\Profiles");
            let firefox_installed = firefox_path.exists();
            browsers.push(BrowserInfo {
                name: "Mozilla Firefox",
                installed: firefox_installed,
                extension_installed: firefox_installed && ext_dir.exists(),
                profile_path: if firefox_installed { Some(firefox_path) } else { None },
            });
        }

        #[cfg(not(target_os = "windows"))]
        {
            // Unix detection fallback
            browsers.push(BrowserInfo {
                name: "Chromium / Chrome",
                installed: true,
                extension_installed: ext_dir.exists(),
                profile_path: None,
            });
        }

        browsers
    }

    pub fn install_system(current_exe: &Path, extension_source_dir: Option<&Path>) -> Result<(), String> {
        let install_dir = Self::get_default_install_dir();
        let bin_dir = install_dir.join("bin");
        let ext_dir = install_dir.join("extension");

        fs::create_dir_all(&bin_dir).map_err(|e| format!("Failed to create {}: {}", bin_dir.display(), e))?;
        fs::create_dir_all(&ext_dir).map_err(|e| format!("Failed to create {}: {}", ext_dir.display(), e))?;

        // Copy binary
        let target_exe = bin_dir.join(if cfg!(windows) { "hmlr.exe" } else { "hmlr" });
        if current_exe != target_exe {
            fs::copy(current_exe, &target_exe)
                .map_err(|e| format!("Failed to copy binary to {}: {}", target_exe.display(), e))?;
        }

        // Copy extension files if available
        if let Some(src_ext) = extension_source_dir {
            if src_ext.exists() {
                Self::copy_dir_all(src_ext, &ext_dir)?;
            }
        }

        Ok(())
    }

    fn copy_dir_all(src: &Path, dst: &Path) -> Result<(), String> {
        fs::create_dir_all(dst).map_err(|e| e.to_string())?;
        for entry in fs::read_dir(src).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let ft = entry.file_type().map_err(|e| e.to_string())?;
            let dst_path = dst.join(entry.file_name());
            if ft.is_dir() {
                Self::copy_dir_all(&entry.path(), &dst_path)?;
            } else {
                fs::copy(entry.path(), dst_path).map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }
}

#[cfg(target_os = "windows")]
fn bra_path(p: PathBuf) -> PathBuf { p }
