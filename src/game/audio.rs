use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source, SourceHandle},
};

pub struct AudioHandles {
    pub eating_noise: SourceHandle,
}

pub fn play_eat_sound(
    audio_handles: &AudioHandles,
    storage: &AssetStorage<Source>,
    output: Option<&Output>,
) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&audio_handles.eating_noise) {
            output.play_once(sound, 1.0);
        }
    }
}
