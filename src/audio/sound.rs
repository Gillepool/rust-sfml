// Rust-SFML - Copyright (c) 2013 Letang Jeremy.
//
// The original software, SFML library, is provided by Laurent Gomila.
//
// This software is provided 'as-is', without any express or implied warranty.
// In no event will the authors be held liable for any damages arising from
// the use of this software.
//
// Permission is granted to anyone to use this software for any purpose,
// including commercial applications, and to alter it and redistribute it
// freely, subject to the following restrictions:
//
// 1. The origin of this software must not be misrepresented; you must not claim
//    that you wrote the original software. If you use this software in a product,
//    an acknowledgment in the product documentation would be appreciated but is
//    not required.
//
// 2. Altered source versions must be plainly marked as such, and must not be
//    misrepresented as being the original software.
//
// 3. This notice may not be removed or altered from any source distribution.
//

use std::mem;
use std::marker::PhantomData;

use audio::{SoundStatus, SoundBufferRef, SoundSource};
use system::Time;
use system::Vector3f;
use system::raw_conv::{Raw, FromRaw};

use csfml_system_sys::{sfBool, sfVector3f};
use csfml_audio_sys as ffi;
use ext::sf_bool_ext::SfBoolExt;

/// Regular sound that can be played in the audio environment.
///
/// `Sound` is the type to use to play sounds.
///
/// It provides:
///
/// - Control (play, pause, stop)
/// - Ability to modify output parameters in real-time (pitch, volume, ...)
/// - 3D spatial features (position, attenuation, ...).
///
/// `Sound` is perfect for playing short sounds that can fit in memory and require no latency,
/// like foot steps or gun shots. For longer sounds, like background musics or long speeches,
/// rather see `Music` (which is based on streaming).
///
/// In order to work, a sound must be given a buffer of audio data to play.
/// Audio data (samples) is stored in `SoundBuffer`, and attached to a sound with the
/// `set_buffer()` function. The buffer object attached to a sound must remain alive as long as
/// the sound uses it. Note that multiple sounds can use the same sound buffer at the same time.
///
/// # Usage example
///
/// ```no_run
/// use sfml::audio::{Sound, SoundBuffer};
///
/// let buffer = SoundBuffer::from_file("sound.wav").unwrap();
/// let mut sound = Sound::with_buffer(&buffer);
/// sound.play();
/// ```
pub struct Sound<'s> {
    sound: *mut ffi::sfSound,
    buffer: PhantomData<&'s SoundBufferRef>,
}

impl<'s> Sound<'s> {
    /// Create a new Sound
    pub fn new() -> Sound<'s> {
        let s = unsafe { ffi::sfSound_create() };
        if s.is_null() {
            panic!("sfSound_create returned null.")
        } else {
            Sound {
                sound: s,
                buffer: PhantomData,
            }
        }
    }

    /// Create a new Sound with a buffer
    pub fn with_buffer(buffer: &SoundBufferRef) -> Sound {
        let s = unsafe { ffi::sfSound_create() };
        if s.is_null() {
            panic!("sfSound_create returned null.")
        } else {
            unsafe {
                ffi::sfSound_setBuffer(s, buffer as *const _ as _);
            }
            Sound {
                sound: s,
                buffer: PhantomData,
            }
        }
    }

    /// Sets whether this sound should loop or not.
    pub fn set_looping(&mut self, looping: bool) {
        unsafe { ffi::sfSound_setLoop(self.sound, sfBool::from_bool(looping)) }
    }

    /// Tell whether or not a sound is in loop mode
    ///
    /// Return true if the sound is looping, false otherwise
    pub fn is_looping(&self) -> bool {
        unsafe { ffi::sfSound_getLoop(self.sound) }.to_bool()
    }

    /// Start or resume playing a sound
    ///
    /// This function starts the sound if it was stopped, resumes
    /// it if it was paused, and restarts it from beginning if it
    /// was it already playing.
    /// This function uses its own thread so that it doesn't block
    /// the rest of the program while the sound is played.
    pub fn play(&mut self) {
        unsafe { ffi::sfSound_play(self.sound) }
    }

    /// Pause a sound
    ///
    /// This function pauses the sound if it was playing,
    /// otherwise (sound already paused or stopped) it has no effect.
    pub fn pause(&mut self) {
        unsafe { ffi::sfSound_pause(self.sound) }
    }

    /// Stop playing a sound
    ///
    /// This function stops the sound if it was playing or paused,
    /// and does nothing if it was already stopped.
    /// It also resets the playing position (unlike pause).
    pub fn stop(&mut self) {
        unsafe { ffi::sfSound_stop(self.sound) }
    }

    /// Get the current status of a sound (stopped, paused, playing)
    ///
    /// Return current status
    pub fn status(&self) -> SoundStatus {
        unsafe { mem::transmute(ffi::sfSound_getStatus(self.sound)) }
    }

    /// Get the current playing position of a sound
    ///
    /// Return the current playing position
    pub fn playing_offset(&self) -> Time {
        unsafe { Time::from_raw(ffi::sfSound_getPlayingOffset(self.sound)) }
    }

    /// Change the current playing position of a sound
    ///
    /// The playing position can be changed when the sound is
    /// either paused or playing.
    ///
    /// # Arguments
    /// * timeOffset - New playing position
    pub fn set_playing_offset(&mut self, time_offset: Time) {
        unsafe { ffi::sfSound_setPlayingOffset(self.sound, time_offset.raw()) }
    }

    /// Set the source buffer containing the audio data to play
    ///
    /// It is important to note that the sound buffer is not copied,
    /// thus the sfSoundBuffer object must remain alive as long
    /// as it is attached to the sound.
    ///
    /// # Arguments
    /// * buffer - Sound buffer to attach to the sound
    pub fn set_buffer(&mut self, buffer: &'s SoundBufferRef) {
        unsafe { ffi::sfSound_setBuffer(self.sound, buffer as *const _ as _) }
    }

    /// Get the audio buffer attached to a sound
    ///
    /// Return an option to Sound buffer attached to the sound or None
    pub fn buffer(&self) -> Option<&SoundBufferRef> {
        unsafe {
            let ptr = ffi::sfSound_getBuffer(self.sound);
            if ptr == ::std::ptr::null() {
                None
            } else {
                Some(&*(ptr as *const SoundBufferRef))
            }
        }
    }
}

impl<'a> Default for Sound<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'s> Clone for Sound<'s> {
    fn clone(&self) -> Self {
        let s = unsafe { ffi::sfSound_copy(self.sound) };
        if s.is_null() {
            panic!("Sound is null");
        } else {
            Sound {
                sound: s,
                buffer: self.buffer,
            }
        }
    }
}

impl<'s> SoundSource for Sound<'s> {
    fn set_pitch(&mut self, pitch: f32) {
        unsafe { ffi::sfSound_setPitch(self.sound, pitch) }
    }
    fn set_volume(&mut self, volume: f32) {
        unsafe { ffi::sfSound_setVolume(self.sound, volume) }
    }
    fn set_position(&mut self, position: &Vector3f) {
        unsafe { ffi::sfSound_setPosition(self.sound, position.raw()) }
    }
    fn set_position3f(&mut self, x: f32, y: f32, z: f32) {
        unsafe { ffi::sfSound_setPosition(self.sound, sfVector3f { x: x, y: y, z: z }) }
    }
    fn set_relative_to_listener(&mut self, relative: bool) {
        unsafe { ffi::sfSound_setRelativeToListener(self.sound, sfBool::from_bool(relative)) }
    }
    fn set_min_distance(&mut self, distance: f32) {
        unsafe { ffi::sfSound_setMinDistance(self.sound, distance) }
    }
    fn set_attenuation(&mut self, attenuation: f32) {
        unsafe { ffi::sfSound_setAttenuation(self.sound, attenuation) }
    }
    fn pitch(&self) -> f32 {
        unsafe { ffi::sfSound_getPitch(self.sound) as f32 }
    }
    fn volume(&self) -> f32 {
        unsafe { ffi::sfSound_getVolume(self.sound) as f32 }
    }
    fn position(&self) -> Vector3f {
        unsafe { Vector3f::from_raw(ffi::sfSound_getPosition(self.sound)) }
    }
    fn is_relative_to_listener(&self) -> bool {
        unsafe { ffi::sfSound_isRelativeToListener(self.sound).to_bool() }
    }
    fn min_distance(&self) -> f32 {
        unsafe { ffi::sfSound_getMinDistance(self.sound) as f32 }
    }
    fn attenuation(&self) -> f32 {
        unsafe { ffi::sfSound_getAttenuation(self.sound) as f32 }
    }
}

impl<'s> Drop for Sound<'s> {
    fn drop(&mut self) {
        unsafe {
            ffi::sfSound_destroy(self.sound);
        }
    }
}
