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
use crate::JsEngineGlobalVar;

pub fn serialize_js_engine_global_var(args: &JsEngineGlobalVar) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported object-type: JsEngineGlobalVar".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_js_engine_global_var(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_js_engine_global_var<W: Write>(args: &JsEngineGlobalVar, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&2)?;
    writer.context().push("name", "String", "writing property");
    writer.write_string("name")?;
    writer.write_string(&args.name)?;
    writer.context().pop();
    writer.context().push("value", "JSON::Value", "writing property");
    writer.write_string("value")?;
    writer.write_json(&args.value)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_js_engine_global_var(args: &[u8]) -> Result<JsEngineGlobalVar, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported object-type: JsEngineGlobalVar".to_string();
    let mut reader = ReadDecoder::new(args, context);
    read_js_engine_global_var(&mut reader)
}

pub fn read_js_engine_global_var<R: Read>(reader: &mut R) -> Result<JsEngineGlobalVar, DecodeError> {
    let mut num_of_fields = reader.read_map_length()?;

    let mut _name: String = String::new();
    let mut _name_set = false;
    let mut _value: JSON::Value = JSON::Value::Null;
    let mut _value_set = false;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "name" => {
                reader.context().push(&field, "String", "type found, reading property");
                _name = reader.read_string()?;
                _name_set = true;
                reader.context().pop();
            }
            "value" => {
                reader.context().push(&field, "JSON::Value", "type found, reading property");
                _value = reader.read_json()?;
                _value_set = true;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_name_set {
        return Err(DecodeError::MissingField("name: String.".to_string()));
    }
    if !_value_set {
        return Err(DecodeError::MissingField("value: JSON.".to_string()));
    }

    Ok(JsEngineGlobalVar {
        name: _name,
        value: _value,
    })
}
