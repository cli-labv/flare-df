//! Módulo de motores de compresión de PDFs
//! =========================================
//!
//! Este módulo proporciona diferentes estrategias de compresión de PDFs
//! usando herramientas externas como qpdf y Ghostscript.

mod traits;
mod qpdf_engine;
mod ghostscript_engine;
mod compression_mode;
mod pipeline;
mod detector;

pub use traits::CompressionEngine;
pub use qpdf_engine::QpdfEngine;
pub use ghostscript_engine::GhostscriptEngine;
pub use compression_mode::CompressionMode;
pub use pipeline::compress_pdf;
pub use detector::{EngineDetector, EngineAvailability};
