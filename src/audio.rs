#![allow(dead_code)]
use sdl2::audio::{AudioCallback, AudioSpecDesired};
use sdl2::AudioSubsystem;
use crate::game::SoundEvent;

struct SoundSynth {
    phase: f32,
    phase_inc: f32,
    volume: f32,
    duration: f32,
    time: f32,
    sound_type: Option<SoundEvent>,
    seed: u32,
}

impl AudioCallback for SoundSynth {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for sample in out.iter_mut() {
            if let Some(sfx) = self.sound_type {
                self.time += 1.0 / 44100.0;
                if self.time >= self.duration {
                    self.sound_type = None;
                    *sample = 0.0;
                    continue;
                }

                let envelope = 1.0 - (self.time / self.duration).clamp(0.0, 1.0);

                let val = match sfx {
                    SoundEvent::BouncePaddle | SoundEvent::BounceWall => {
                        let freq = if sfx == SoundEvent::BouncePaddle { 520.0 } else { 380.0 };
                        self.phase_inc = freq / 44100.0;
                        self.phase = (self.phase + self.phase_inc) % 1.0;
                        (if self.phase < 0.5 { 0.15 } else { -0.15 }) * envelope
                    }
                    SoundEvent::LaserFire => {
                        let freq = 1200.0 - (self.time / self.duration) * 800.0;
                        self.phase_inc = freq / 44100.0;
                        self.phase = (self.phase + self.phase_inc) % 1.0;
                        (if self.phase < 0.5 { 0.2 } else { -0.2 }) * envelope
                    }
                    SoundEvent::BrickDestroy => {
                        let freq = 700.0 - (self.time / self.duration) * 450.0;
                        self.phase_inc = freq / 44100.0;
                        self.phase = (self.phase + self.phase_inc) % 1.0;
                        (if self.phase < 0.5 { 0.2 } else { -0.2 }) * envelope
                    }
                    SoundEvent::Explosion => {
                        self.seed = self.seed.wrapping_mul(1664525).wrapping_add(1013904223);
                        let noise = ((self.seed >> 16) as f32 / 65535.0) * 2.0 - 1.0;
                        noise * 0.3 * envelope
                    }
                    SoundEvent::PowerupPickup => {
                        let step = (self.time / self.duration * 3.0) as u32;
                        let freq = match step {
                            0 => 523.25,
                            1 => 659.25,
                            _ => 784.00,
                        };
                        self.phase_inc = freq / 44100.0;
                        self.phase = (self.phase + self.phase_inc) % 1.0;
                        (self.phase * std::f32::consts::TAU).sin() * 0.25 * envelope
                    }
                    SoundEvent::Warp => {
                        let freq = 400.0 + (self.time * 20.0).sin() * 200.0;
                        self.phase_inc = freq / 44100.0;
                        self.phase = (self.phase + self.phase_inc) % 1.0;
                        (self.phase * std::f32::consts::TAU).sin() * 0.2 * envelope
                    }
                    SoundEvent::GameOver => {
                        let freq = 300.0 - (self.time / self.duration) * 180.0;
                        self.phase_inc = freq / 44100.0;
                        self.phase = (self.phase + self.phase_inc) % 1.0;
                        (if self.phase < 0.5 { 0.2 } else { -0.2 }) * envelope
                    }
                    SoundEvent::Victory => {
                        let step = (self.time / self.duration * 4.0) as u32;
                        let freq = match step {
                            0 => 440.0,
                            1 => 554.37,
                            2 => 659.25,
                            _ => 880.0,
                        };
                        self.phase_inc = freq / 44100.0;
                        self.phase = (self.phase + self.phase_inc) % 1.0;
                        (self.phase * std::f32::consts::TAU).sin() * 0.25 * envelope
                    }
                };

                *sample = val;
            } else {
                *sample = 0.0;
            }
        }
    }
}

pub struct AudioEngine {
    device: Option<sdl2::audio::AudioDevice<SoundSynth>>,
}

impl AudioEngine {
    pub fn new(audio_subsystem: &AudioSubsystem) -> Self {
        let desired_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1),
            samples: None,
        };

        let device = audio_subsystem
            .open_playback(None, &desired_spec, |_spec| SoundSynth {
                phase: 0.0,
                phase_inc: 0.0,
                volume: 0.25,
                duration: 0.1,
                time: 0.0,
                sound_type: None,
                seed: 12345,
            })
            .ok();

        if let Some(ref dev) = device {
            dev.resume();
        }

        Self { device }
    }

    pub fn play(&mut self, sfx: SoundEvent) {
        if let Some(ref mut dev) = self.device {
            let mut lock = dev.lock();
            lock.sound_type = Some(sfx);
            lock.time = 0.0;
            lock.duration = match sfx {
                SoundEvent::BouncePaddle | SoundEvent::BounceWall => 0.08,
                SoundEvent::LaserFire => 0.1,
                SoundEvent::BrickDestroy => 0.12,
                SoundEvent::Explosion => 0.3,
                SoundEvent::PowerupPickup => 0.2,
                SoundEvent::Warp => 0.35,
                SoundEvent::GameOver => 0.6,
                SoundEvent::Victory => 0.8,
            };
        }
    }
}
