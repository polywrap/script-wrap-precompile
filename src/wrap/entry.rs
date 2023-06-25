use polywrap_wasm_rs::{
    abort,
    invoke,
    InvokeArgs,
};
use crate::run;

#[no_mangle]
pub extern "C" fn _wrap_invoke(method_size: u32, args_size: u32, _env_size: u32) -> bool {
    // Ensure the abort handler is properly setup
    abort::wrap_abort_setup();

    let args: InvokeArgs = invoke::wrap_invoke_args(method_size, args_size);
    let result = run(&args.method, &args.args);

    if let Err(err) = result {
        invoke::wrap_invoke_error(format!("Invocation error for method: {}, error: {}", args.method, err));
        return false;
    }
    invoke::wrap_invoke_result(result.unwrap());
    return true;
}
