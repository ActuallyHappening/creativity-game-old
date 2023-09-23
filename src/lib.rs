pub fn init_debug_tools() {
    #[cfg(not(target_arch = "wasm32"))]
    tracing_subscriber::fmt::init();

    #[cfg(target_arch = "wasm32")]
    {
        use tracing_subscriber::prelude::*;
        console_error_panic_hook::set_once();
        tracing_subscriber::registry::Registry::default()
            .with(tracing_wasm::WASMLayer::new(
                tracing_wasm::WASMLayerConfig::default(),
            ))
            .init();
    }
}