use std::convert::TryFrom;
use polywrap_wasm_rs::{
    BigInt,
    BigNumber,
    Map,
    Context,
    DecodeError,
    EncodeError,
    Read,
    ReadDecoder,
    Write,
    WriteEncoder,
    JSON,
};
use crate::JsEngineEvalResult;

pub fn serialize_js_engine_eval_result(args: &JsEngineEvalResult) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported object-type: JsEngineEvalResult".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_js_engine_eval_result(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_js_engine_eval_result<W: Write>(args: &JsEngineEvalResult, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&2)?;
    writer.context().push("value", "Option<JSON::Value>", "writing property");
    writer.write_string("value")?;
    writer.write_optional_json(&args.value)?;
    writer.context().pop();
    writer.context().push("error", "Option<String>", "writing property");
    writer.write_string("error")?;
    writer.write_optional_string(&args.error)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_js_engine_eval_result(args: &[u8]) -> Result<JsEngineEvalResult, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported object-type: JsEngineEvalResult".to_string();
    let mut reader = ReadDecoder::new(args, context);
    read_js_engine_eval_result(&mut reader)
}

pub fn read_js_engine_eval_result<R: Read>(reader: &mut R) -> Result<JsEngineEvalResult, DecodeError> {
    let mut num_of_fields = reader.read_map_length()?;

    let mut _value: Option<JSON::Value> = None;
    let mut _error: Option<String> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "value" => {
                reader.context().push(&field, "Option<JSON::Value>", "type found, reading property");
                _value = reader.read_optional_json()?;
                reader.context().pop();
            }
            "error" => {
                reader.context().push(&field, "Option<String>", "type found, reading property");
                _error = reader.read_optional_string()?;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }

    Ok(JsEngineEvalResult {
        value: _value,
        error: _error,
    })
}
