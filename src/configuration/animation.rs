// src/configuration/animation.rs

pub struct AnimationConfig {
    pub blur_enabled: bool,
    pub fade_in_enabled: bool,
}

impl AnimationConfig {
    pub fn new() -> Self {
        AnimationConfig {
            blur_enabled: true,  // Default value
            fade_in_enabled: true, // Default value
        }
    }

    pub fn animation_default_settings(&self) -> (bool, bool) {
        (self.blur_enabled, self.fade_in_enabled)
    }
}
