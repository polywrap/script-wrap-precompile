use serde::{Serialize, Deserialize};
use polywrap_wasm_rs::{
    BigInt,
    BigNumber,
    Map,
    Read,
    Write,
    JSON,
    subinvoke,
};
pub mod serialization;
pub use serialization::{
    deserialize_eval_result,
    serialize_eval_args,
    ArgsEval
};

use crate::JsEngineEvalResult;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JsEngineModule {}

impl JsEngineModule {
    pub const URI: &'static str = "ipfs/QmSkuPz5kgMQQKQA4FgJV3GiNnXVkeSDPyUBF2HLeArfEv";

    pub fn new() -> JsEngineModule {
        JsEngineModule {}
    }

    pub fn eval(args: &ArgsEval) -> Result<JsEngineEvalResult, String> {
        let uri = JsEngineModule::URI;
        let args = serialize_eval_args(args).map_err(|e| e.to_string())?;
        let result = subinvoke::wrap_subinvoke(
            uri,
            "eval",
            args,
        )?;
        deserialize_eval_result(result.as_slice()).map_err(|e| e.to_string())
    }
}
