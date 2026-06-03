//! Shared API for fjs native extensions.

use rquickjs::Ctx;

pub use linkme;
pub use rquickjs;

/// A global initialization function contributed by a native extension.
pub type NativeGlobalInit = fn(&Ctx<'_>) -> rquickjs::Result<()>;

/// Factory for native extension registrations.
pub type NativeExtensionFactory = fn() -> NativeExtension;

/// Native extension hooks that can add host-provided behavior to a context.
#[derive(Debug, Default, Clone)]
pub struct NativeExtension {
    globals: Vec<NativeGlobalInit>,
}

impl NativeExtension {
    /// Creates an empty native extension.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a global initialization function to the extension.
    pub fn with_global(mut self, init: NativeGlobalInit) -> Self {
        self.globals.push(init);
        self
    }

    /// Returns global initialization functions registered by this extension.
    pub fn globals(&self) -> &[NativeGlobalInit] {
        &self.globals
    }
}

/// Linker-collected native extensions.
#[linkme::distributed_slice]
pub static NATIVE_EXTENSIONS: [NativeExtensionFactory];

/// Returns every native extension linked into the final fjs binary.
pub fn registered_native_extensions() -> impl Iterator<Item = NativeExtension> {
    NATIVE_EXTENSIONS.iter().map(|factory| factory())
}
