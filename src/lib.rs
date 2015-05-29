extern crate rustc_serialize;
extern crate time;
extern crate bufstream;

mod macros;
pub mod error;
pub mod version;
pub mod reply;
pub mod status;
pub mod replaygain;
pub mod song;
pub mod output;
pub mod playlist;
pub mod search;

mod traits;
pub mod client;

pub use client::Client;
pub use status::Status;
pub use replaygain::ReplayGain;
pub use version::Version;
pub use song::Song;
pub use playlist::Playlist;
pub use output::Output;
pub use search::{Term, Query, Clause};
