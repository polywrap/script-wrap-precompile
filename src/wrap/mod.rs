pub mod entry;
pub mod imported;
pub use imported::js_engine_eval_result::JsEngineEvalResult;
pub use imported::js_engine_global_var::JsEngineGlobalVar;
pub use imported::js_engine_module::JsEngineModule;
pub mod module;
pub use module::{
    Module,
    ModuleTrait,
};

// Override print!(...) & println!(...) macros
#[macro_export]
macro_rules! println { ($($args:tt)*) => { polywrap_wasm_rs::wrap_debug_log(format!($($args)*).as_str()); } }
#[macro_export]
macro_rules! print { ($($args:tt)*) => { polywrap_wasm_rs::wrap_debug_log(format!($($args)*).as_str()); } }
