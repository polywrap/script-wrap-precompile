use serde::{Serialize, Deserialize};
pub mod serialization;
use polywrap_wasm_rs::{
    BigInt,
    BigNumber,
    Map,
    DecodeError,
    EncodeError,
    Read,
    Write,
    JSON,
};
pub use serialization::{
    deserialize_js_engine_eval_result,
    read_js_engine_eval_result,
    serialize_js_engine_eval_result,
    write_js_engine_eval_result
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JsEngineEvalResult {
    pub value: Option<JSON::Value>,
    pub error: Option<String>,
}

impl JsEngineEvalResult {
    pub const URI: &'static str = "ipfs/QmSkuPz5kgMQQKQA4FgJV3GiNnXVkeSDPyUBF2HLeArfEv";

    pub fn new() -> JsEngineEvalResult {
        JsEngineEvalResult {
            value: None,
            error: None,
        }
    }

    pub fn to_buffer(args: &JsEngineEvalResult) -> Result<Vec<u8>, EncodeError> {
        serialize_js_engine_eval_result(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<JsEngineEvalResult, DecodeError> {
        deserialize_js_engine_eval_result(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &JsEngineEvalResult, writer: &mut W) -> Result<(), EncodeError> {
        write_js_engine_eval_result(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<JsEngineEvalResult, DecodeError> {
        read_js_engine_eval_result(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
