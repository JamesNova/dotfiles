#![warn(missing_docs)]

//! # raur
//!
//! raur is a library for interacting with the
//! [aurweb RPC Interface](https://aur.archlinux.org/rpc).
//!
//! See also the [Arch wiki page](https://aur.archlinux.org/rpc.php) for more information.
//!
//! # Example
//!
//! ```
//! # #[tokio::main]
//! # async fn main() -> Result<(), raur::Error> {
//! # #[cfg(feature = "async")]
//! # {
//! use raur::Raur;
//!
//! let raur = raur::Handle::new();
//!
//! // Use `search` to search using keywords (multiple strategies available)
//! let pkgs = raur.search("pacman").await?;
//! assert!(pkgs.len() > 10);
//!
//! for pkg in pkgs {
//!     println!("{:<30}{}", pkg.name, pkg.version);
//! }
//!
//! // Use `info` to get info about a list of packages. Not-found packages are silently ignored.
//! let pkgs = raur.info(&["spotify", "discord-canary"]).await?;
//! assert_eq!(pkgs.len(), 2);
//!
//! for pkg in pkgs {
//!     println!("{:<30}{}", pkg.name, pkg.version);
//! }
//! # }
//! # Ok(())
//! # }
//! ```

/// Non async handle
#[cfg(feature = "blocking")]
pub mod blocking;
mod cache;
mod error;
#[cfg(feature = "async")]
mod handle;
mod raur;

pub use crate::cache::*;
pub use crate::error::*;
#[cfg(feature = "async")]
pub use crate::handle::*;
pub use crate::raur::*;
