use polywrap_wasm_rs::{
    abort,
    invoke,
    InvokeArgs,
};
use crate::run_js_wrap;

#[no_mangle]
pub extern "C" fn _wrap_invoke(method_size: u32, args_size: u32, env_size: u32) -> bool {
    // Ensure the abort handler is properly setup
    abort::wrap_abort_setup();

    let args: InvokeArgs = invoke::wrap_invoke_args(method_size, args_size);
    let result = run_js_wrap(&args.method, &args.args);
    invoke::wrap_invoke_result(result);
    return true;
}
