mod deno;

use crate::api;

api!(
    /// Provides access to the scripts API.
    ///
    /// The scripts executed by this API must be placed in `assets/mods`.
    ScriptsApi
);
