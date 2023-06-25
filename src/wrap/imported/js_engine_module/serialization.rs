use serde::{Serialize, Deserialize};
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
use crate::JsEngineGlobalVar;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsEval {
    pub src: String,
}

pub fn deserialize_eval_args(args: &[u8]) -> Result<ArgsEval, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported module-type: eval Args".to_string();

    let mut reader = ReadDecoder::new(args, context);
    let mut num_of_fields = reader.read_map_length()?;

    let mut _src: String = String::new();
    let mut _src_set = false;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "src" => {
                reader.context().push(&field, "String", "type found, reading argument");
                _src = reader.read_string()?;
                _src_set = true;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_src_set {
        return Err(DecodeError::MissingField("src: String.".to_string()));
    }

    Ok(ArgsEval {
        src: _src,
    })
}

pub fn serialize_eval_args(args: &ArgsEval) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported module-type: eval Args".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_eval_args(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_eval_args<W: Write>(args: &ArgsEval, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&1)?;
    writer.context().push("src", "String", "writing property");
    writer.write_string("src")?;
    writer.write_string(&args.src)?;
    writer.context().pop();
    Ok(())
}

pub fn serialize_eval_result(result: &JsEngineEvalResult) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported module-type: eval Result".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_eval_result(result, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_eval_result<W: Write>(result: &JsEngineEvalResult, writer: &mut W) -> Result<(), EncodeError> {
    writer.context().push("eval", "JsEngineEvalResult", "writing result");
    JsEngineEvalResult::write(&result, writer)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_eval_result(result: &[u8]) -> Result<JsEngineEvalResult, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported module-type: eval Result".to_string();
    let mut reader = ReadDecoder::new(result, context);

    reader.context().push("eval", "JsEngineEvalResult", "reading function output");
    let object = JsEngineEvalResult::read(&mut reader)?;
    let res = object;
    reader.context().pop();
    Ok(res)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsEvalWithGlobals {
    pub src: String,
    pub globals: Vec<JsEngineGlobalVar>,
}

pub fn deserialize_eval_with_globals_args(args: &[u8]) -> Result<ArgsEvalWithGlobals, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported module-type: eval_with_globals Args".to_string();

    let mut reader = ReadDecoder::new(args, context);
    let mut num_of_fields = reader.read_map_length()?;

    let mut _src: String = String::new();
    let mut _src_set = false;
    let mut _globals: Vec<JsEngineGlobalVar> = vec![];
    let mut _globals_set = false;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "src" => {
                reader.context().push(&field, "String", "type found, reading argument");
                _src = reader.read_string()?;
                _src_set = true;
                reader.context().pop();
            }
            "globals" => {
                reader.context().push(&field, "Vec<JsEngineGlobalVar>", "type found, reading argument");
                _globals = reader.read_array(|reader| {
                    let object = JsEngineGlobalVar::read(reader)?;
                    Ok(object)
                })?;
                _globals_set = true;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_src_set {
        return Err(DecodeError::MissingField("src: String.".to_string()));
    }
    if !_globals_set {
        return Err(DecodeError::MissingField("globals: [JsEngine_GlobalVar].".to_string()));
    }

    Ok(ArgsEvalWithGlobals {
        src: _src,
        globals: _globals,
    })
}

pub fn serialize_eval_with_globals_args(args: &ArgsEvalWithGlobals) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported module-type: eval_with_globals Args".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_eval_with_globals_args(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_eval_with_globals_args<W: Write>(args: &ArgsEvalWithGlobals, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&2)?;
    writer.context().push("src", "String", "writing property");
    writer.write_string("src")?;
    writer.write_string(&args.src)?;
    writer.context().pop();
    writer.context().push("globals", "Vec<JsEngineGlobalVar>", "writing property");
    writer.write_string("globals")?;
    writer.write_array(&args.globals, |writer, item| {
        JsEngineGlobalVar::write(item, writer)
    })?;
    writer.context().pop();
    Ok(())
}

pub fn serialize_eval_with_globals_result(result: &JsEngineEvalResult) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported module-type: eval_with_globals Result".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_eval_with_globals_result(result, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_eval_with_globals_result<W: Write>(result: &JsEngineEvalResult, writer: &mut W) -> Result<(), EncodeError> {
    writer.context().push("evalWithGlobals", "JsEngineEvalResult", "writing result");
    JsEngineEvalResult::write(&result, writer)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_eval_with_globals_result(result: &[u8]) -> Result<JsEngineEvalResult, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported module-type: eval_with_globals Result".to_string();
    let mut reader = ReadDecoder::new(result, context);

    reader.context().push("evalWithGlobals", "JsEngineEvalResult", "reading function output");
    let object = JsEngineEvalResult::read(&mut reader)?;
    let res = object;
    reader.context().pop();
    Ok(res)
}
