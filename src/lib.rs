//! This crate offers Rust bindings to [KaTeX](https://katex.org).
//! This allows you to render LaTeX equations to HTML.
//!
//! # Usage
//!
//! Add this to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! katex = "0.4.0-alpha.1"
//! ```
//!
//! # Examples
//!
//! ```
//! let html = katex::render("E = mc^2").unwrap();
//!
//! let opts = katex::Opts::builder().display_mode(true).build().unwrap();
//! let html_in_display_mode = katex::render_with_opts("E = mc^2", &opts).unwrap();
//! ```

#![feature(generic_associated_types)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

use core::cell::RefCell;

pub mod error;
pub use error::{Error, Result};

pub mod opts;
pub use opts::{Opts, OptsBuilder, OutputType};

mod js_engine;
use js_engine::{Engine, JsEngine, JsValue};

/// KaTeX JS source code.
const KATEX_SRC: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/vendor/katex.min.js"));
/// mhchem JS source code.
const MHCHEM_SRC: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/vendor/contrib/mhchem.min.js"
));

thread_local! {
    /// Per thread JS Engine used to render KaTeX.
    static KATEX: Result<RefCell<Engine>> = init_katex();
}

/// Initialize KaTeX js environment.
fn init_katex<Engine>() -> Result<RefCell<Engine>>
where
    Engine: JsEngine,
{
    let engine = Engine::new()?;
    engine.eval(KATEX_SRC)?;
    engine.eval(MHCHEM_SRC)?;
    engine.eval(
        "function renderToString(input, opts) { return katex.renderToString(input, opts); }",
    )?;
    Ok(RefCell::new(engine))
}

/// Render LaTeX equation to HTML using specified [engine](`JsEngine`) and [options](`Opts`).
#[inline]
fn render_inner<'a, Engine>(
    engine: &'a mut Engine,
    input: &str,
    opts: impl AsRef<Opts>,
) -> Result<String>
where
    Engine: JsEngine,
{
    use core::iter;

    let input = engine.from_string(input.to_owned());
    let opts = opts.as_ref().to_js_value(engine);
    let args = iter::once(input).chain(iter::once(opts));
    engine.call_function("renderToString", args)?.into_string()
}

/// Render LaTeX equation to HTML with additional [options](`Opts`).
pub fn render_with_opts(input: &str, opts: impl AsRef<Opts>) -> Result<String> {
    KATEX.with(|engine| {
        engine
            .as_ref()
            .map_err(|e| e.clone())
            .and_then(|engine| render_inner(&mut *engine.borrow_mut(), input, opts))
    })
}

/// Render LaTeX equation to HTML.
#[inline]
pub fn render(input: &str) -> Result<String> {
    render_with_opts(input, Opts::default())
}

#[cfg(test)]
mod tests;
