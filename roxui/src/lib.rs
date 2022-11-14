mod util;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Debug)]
pub struct ErrorSpan {
    // pub value: String, // This won't work. See working example below.
    pub source: String,
    pub line: usize,
}

impl From<roxc::ErrorReport> for ErrorSpan {
    fn from(source: roxc::ErrorReport) -> Self {
        ErrorSpan {
            source: source.source,
            line: source.line,
        }
    }
}

// impl Into<LocalErrorSpan> for roxc::ErrorSpan {
//     fn into(self) -> LocalErrorSpan {
//         LocalErrorSpan {
//             source: self.source,
//             line: self.line,
//         }
//     }
// }

#[wasm_bindgen]
pub fn set_debug() {
    util::set_panic_hook();
}

#[wasm_bindgen]
pub fn run_code(code: &str) -> Result<String, ErrorSpan> {
    Ok(roxc::run(code)?)
}
