//! GJEB for Horizon: brightness past the video settings' 100%.
//!
//! The Java mod this ports (GJEB, GammaJustExtremeBright) widens the brightness slider's range so
//! the player can reach 1000% and see in the dark. Horizon has no such option, so this mod keeps its
//! own: a slider on its page in Options > Mods, a toggle and a key that turn full brightness on and
//! off. The value is pushed into the renderer's lightmap gamma every frame by the mixin beside this
//! crate, and the game's own brightness setting and `options.txt` are never touched.
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use mod_api::{
    ClientHost, ClientTickPostEvent, Hooks, KeyConflictContext, KeyRegistration, Registrar,
    RegistrationContext, SettingSpec,
};

/// The mod's id: its settings folder, its key's line and the namespace of its resources.
pub const ID: &str = "gjeb";
/// The key's id within the mod.
pub const KEY: &str = "toggle";
pub const DEFAULT_KEY: &str = "key.keyboard.g";
/// The slider's ends and its default, in percent of vanilla's Bright.
pub const MIN_PERCENT: f64 = 100.0;
pub const MAX_PERCENT: f64 = 1000.0;
pub const DEFAULT_PERCENT: f64 = 1000.0;
/// The slider's step, in percent.
pub const STEP_PERCENT: f64 = 10.0;

/// The gamma the mixin puts into the renderer's lightmap while the mod is on: zero bits when the
/// player's own brightness applies.
static OVERRIDE: AtomicU32 = AtomicU32::new(0);

/// The gamma the mixin should apply this frame, or `None` while the player's own brightness does.
pub fn override_gamma() -> Option<f32> {
    let bits = OVERRIDE.load(Ordering::Relaxed);
    (bits != 0).then(|| f32::from_bits(bits))
}

/// Sets (or clears, with zero) the gamma the mixin applies.
pub fn set_override_gamma(gamma: f32) {
    let bits = if gamma.is_finite() && gamma > 0.0 {
        gamma.to_bits()
    } else {
        0
    };
    OVERRIDE.store(bits, Ordering::Relaxed);
}

/// The settings the player picks on GJEB's page in Options > Mods.
pub fn settings() -> [SettingSpec; 2] {
    [
        SettingSpec::toggle("enabled", "gjeb.setting.enabled", true)
            .tooltip("gjeb.setting.enabled.tooltip"),
        SettingSpec::range(
            "brightness",
            "gjeb.setting.brightness",
            MIN_PERCENT,
            MAX_PERCENT,
            STEP_PERCENT,
            DEFAULT_PERCENT,
        )
        .tooltip("gjeb.setting.brightness.tooltip"),
    ]
}

/// The gamma a brightness in percent means: 100% is vanilla's Bright (1.0), 1000% is 10.0.
pub fn gamma_for(percent: f64) -> f32 {
    (percent.clamp(MIN_PERCENT, MAX_PERCENT) / 100.0) as f32
}

#[derive(Default)]
struct Gjeb {
    /// Whether the toggle key was down at the last tick, for the edge.
    pressed: AtomicBool,
}

impl Gjeb {
    /// The gamma the settings ask for, none while the mod is switched off.
    fn refresh(&self, host: &dyn ClientHost) {
        let gamma = if host.on(ID) && host.setting_bool("enabled").unwrap_or(true) {
            let percent = host
                .setting_f64("brightness")
                .unwrap_or(DEFAULT_PERCENT)
                .clamp(MIN_PERCENT, MAX_PERCENT);
            gamma_for(percent)
        } else {
            0.0
        };
        set_override_gamma(gamma);
    }

    /// Follows the toggle key: a press flips the stored `enabled` setting.
    fn follow_key(&self, host: &mut dyn ClientHost) {
        let down = host.on(ID) && host.is_down(ID, KEY);
        let before = self.pressed.swap(down, Ordering::Relaxed);
        if down && !before {
            let now = !host.setting_bool("enabled").unwrap_or(true);
            host.set_setting(ID, "enabled", if now { "true" } else { "false" });
        }
    }
}

#[mod_api::horizon_mod]
impl Hooks for Gjeb {
    fn register(&self, r: &mut dyn Registrar, _: &RegistrationContext) {
        r.register_key_mapping(
            KeyRegistration::new(KEY, DEFAULT_KEY)
                .label("gjeb.key.toggle")
                .context(KeyConflictContext::InGame),
        );
        for setting in settings() {
            // Only a malformed spec is refused, and the tests check every spec.
            let _ = r.register_setting(setting);
        }
    }

    fn client_tick_post(&self, host: &mut dyn ClientHost, _: &mut ClientTickPostEvent) {
        self.follow_key(host);
        self.refresh(host);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_setting_is_one_the_game_takes() {
        for setting in settings() {
            assert_eq!(setting.check(), Ok(()), "{}", setting.key);
        }
    }

    #[test]
    fn a_percent_means_the_lightmap_gamma() {
        assert_eq!(gamma_for(100.0), 1.0);
        assert_eq!(gamma_for(1000.0), 10.0);
        // Outside the slider's ends the ends apply.
        assert_eq!(gamma_for(0.0), 1.0);
        assert_eq!(gamma_for(9000.0), 10.0);
    }

    #[test]
    fn the_override_is_read_back_and_cleared() {
        set_override_gamma(7.5);
        assert_eq!(override_gamma(), Some(7.5));
        set_override_gamma(0.0);
        assert_eq!(override_gamma(), None);
        set_override_gamma(f32::NAN);
        assert_eq!(override_gamma(), None);
    }
}
