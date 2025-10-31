use threadweaver_core::resources::PurchasedUpgrades;

#[cfg(target_arch = "wasm32")]
mod web_storage {
    use super::*;
    use wasm_bindgen::JsCast;
    use web_sys::Storage;

    const CURRENCY_KEY: &str = "threadweaver_currency";
    const UPGRADES_KEY: &str = "threadweaver_upgrades";

    fn local_storage() -> Option<Storage> {
        web_sys::window()?.local_storage().ok().flatten()
    }

    pub fn load_currency() -> u32 {
        local_storage()
            .and_then(|storage| storage.get_item(CURRENCY_KEY).ok().flatten())
            .and_then(|value| value.parse::<u32>().ok())
            .unwrap_or_default()
    }

    pub fn save_currency(value: u32) {
        if let Some(storage) = local_storage() {
            let _ = storage.set_item(CURRENCY_KEY, &value.to_string());
        }
    }

    pub fn load_upgrades() -> PurchasedUpgrades {
        local_storage()
            .and_then(|storage| storage.get_item(UPGRADES_KEY).ok().flatten())
            .and_then(|json| serde_json::from_str(&json).ok())
            .unwrap_or_default()
    }

    pub fn save_upgrades(upgrades: &PurchasedUpgrades) {
        if let Some(storage) = local_storage() {
            if let Ok(json) = serde_json::to_string(upgrades) {
                let _ = storage.set_item(UPGRADES_KEY, &json);
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod native_storage {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    const FILE_NAME: &str = "threadweaver_state.json";

    fn storage_path() -> PathBuf {
        std::env::var_os("THREADWEAVER_DATA")
            .map(PathBuf::from)
            .unwrap_or_else(|| std::env::temp_dir().join(FILE_NAME))
    }

    pub fn load_currency() -> u32 {
        load_state().map(|state| state.currency).unwrap_or_default()
    }

    pub fn save_currency(value: u32) {
        let mut state = load_state().unwrap_or_default();
        state.currency = value;
        let _ = save_state(state);
    }

    pub fn load_upgrades() -> PurchasedUpgrades {
        load_state().map(|state| state.upgrades).unwrap_or_default()
    }

    pub fn save_upgrades(upgrades: &PurchasedUpgrades) {
        let mut state = load_state().unwrap_or_default();
        state.upgrades = upgrades.clone();
        let _ = save_state(state);
    }

    #[derive(Default, serde::Serialize, serde::Deserialize)]
    struct NativeState {
        currency: u32,
        upgrades: PurchasedUpgrades,
    }

    fn load_state() -> Option<NativeState> {
        let path = storage_path();
        let data = fs::read_to_string(path).ok()?;
        serde_json::from_str(&data).ok()
    }

    fn save_state(state: NativeState) -> std::io::Result<()> {
        let data = serde_json::to_string(&state).unwrap();
        fs::write(storage_path(), data)
    }
}

#[cfg(target_arch = "wasm32")]
pub use web_storage::{load_currency, load_upgrades, save_currency, save_upgrades};

#[cfg(not(target_arch = "wasm32"))]
pub use native_storage::{load_currency, load_upgrades, save_currency, save_upgrades};
