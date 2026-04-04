//! FLARE-DF - Compresor de PDFs ultrarrápido y 100% lossless
//! ============================================================
//!
//! Punto de entrada principal de la aplicación.

mod cli;
mod config;
mod core;
mod engines;
mod models;
mod utils;

use anyhow::Result;
use cli::app::FlareApp;

fn main() -> Result<()> {
    let app = FlareApp::new();
    app.run()
}
