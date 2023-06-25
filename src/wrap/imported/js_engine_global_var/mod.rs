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
    deserialize_js_engine_global_var,
    read_js_engine_global_var,
    serialize_js_engine_global_var,
    write_js_engine_global_var
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JsEngineGlobalVar {
    pub name: String,
    pub value: JSON::Value,
}

impl JsEngineGlobalVar {
    pub const URI: &'static str = "ipfs/Qmbokxv3S2UFvkM569Gu4XCi4KvVCn138U7xBFCxfGQipo";

    pub fn new() -> JsEngineGlobalVar {
        JsEngineGlobalVar {
            name: String::new(),
            value: JSON::Value::Null,
        }
    }

    pub fn to_buffer(args: &JsEngineGlobalVar) -> Result<Vec<u8>, EncodeError> {
        serialize_js_engine_global_var(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<JsEngineGlobalVar, DecodeError> {
        deserialize_js_engine_global_var(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &JsEngineGlobalVar, writer: &mut W) -> Result<(), EncodeError> {
        write_js_engine_global_var(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<JsEngineGlobalVar, DecodeError> {
        read_js_engine_global_var(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
