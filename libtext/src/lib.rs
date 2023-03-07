// transcribe_me re-export, module which will transcribe select mono audio files and return
// a TypedNarrative
mod transcribe_me;
pub use transcribe_me::transcribe_audio_file;

// record_me re-export; module which handles code that will record a user and then transcribe the
// audio.
mod record_me;
pub use record_me::record_audio;
