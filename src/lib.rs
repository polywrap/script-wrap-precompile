mod wrap;
use serde_json::Value;
use wrap::*;
use wrap::JsEngineModule;
use wrap::imported::ArgsEvalWithGlobals;

const USER_MODULE: &[u8; 64000] = include_bytes!("./externs/extern-64KB.bin");

pub fn run(method: &str, args: &[u8]) -> Result<Vec<u8>, String> {
    _run(method, args, &|| {
        // `.to_vec()` here, just like the '1' character in the extern.bin's is needed to ensure that the compiler doesn't
        // optimize away the USER_MODULE constant.
        get_engine_with_user_code(&USER_MODULE.to_vec())
    })
}

fn _run(method: &str, args: &[u8], load_user_module: &dyn Fn() -> Result<EngineWithUserCode, String>) -> Result<Vec<u8>, String> {
    let injected = load_user_module()?;

    let value: rmpv::Value = rmp_serde::from_slice(&args).unwrap();
    let json = serde_json::to_value(&value).unwrap();

    let result = JsEngineModule::eval_with_globals(&ArgsEvalWithGlobals {
        src: injected.user_code,
        globals: vec![
            JsEngineGlobalVar {
                name: "__wrap_method".to_string(),
                value: Value::String(method.to_string()),
            },
            JsEngineGlobalVar {
                name: "__wrap_args".to_string(),
                value: json,
            },
    ]}, &injected.engine_url)?;

    let result = result.value.ok_or("Value retuned from Engine is None.")?;

    let result = json_to_msgpack(&result.to_string());

    Ok(result)
}

fn msgpack_to_json(bytes: &[u8]) -> String {
    let value: rmpv::Value = rmp_serde::from_slice(&bytes).unwrap();
    serde_json::to_string(&value).unwrap()
}

fn json_to_msgpack(string: &str) -> Vec<u8> {
    let value: serde_json::Value = serde_json::from_str(string).unwrap();
    rmp_serde::encode::to_vec(&value).unwrap()
}

#[derive(Debug, PartialEq)]
struct EngineWithUserCode {
    pub engine_url: String,
    pub user_code: String,
}

fn get_engine_with_user_code(injected_bytes: &[u8]) -> Result<EngineWithUserCode, String> {
    let null_pos = injected_bytes.iter().position(|&x| x == 0).ok_or("No null byte found in engine url.")?;
    let engine = String::from_utf8(injected_bytes[..null_pos].to_vec()).unwrap();
    
    let injected_bytes = &injected_bytes[null_pos + 1..];
    let null_pos = injected_bytes.iter().position(|&x| x == 0).ok_or("No null byte found in user code.")?;

    let user_code = String::from_utf8(injected_bytes[..null_pos].to_vec()).unwrap();
  
    Ok(EngineWithUserCode {
        engine_url: engine,
        user_code,
    })
}

#[cfg(test)]
mod tests {
    use std::fs;

    use polywrap_client::msgpack;
    use serde::{Serialize, Deserialize};
    mod test_utils;
    use test_utils::get_client_with_module;

    use crate::{tests::test_utils::{replace_user_module, invoke_client, load_wrap}, get_engine_with_user_code, EngineWithUserCode, USER_MODULE};

    #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
    pub struct MockType {
        pub prop: String,
    }

    // #[test]
    // fn integration_sanity() {
    //     let user_file = "./src/test.js";
    //     let user_code = fs::read_to_string(user_file).unwrap();

    //     let (_manifest, mut module) = load_wrap("./build");
    //     replace_user_module(&mut module, &user_code, "wrap://ipfs/QmUhxYCRG3C7siCydRUqXqz7FZWCMx1kfmMcAMZq52WTzU".to_string());

    //     let client = get_client_with_module(&module);

    //     let result = invoke_client("mock/test", "doStuff", &msgpack::msgpack!({
    //         "prop": "arg1"
    //     }), &client);

    //     let result: MockType = rmp_serde::from_slice(&result).unwrap();

    //     assert_eq!(result, MockType {
    //         prop: String::from("Hello world"),
    //     });
    // }

    #[test]
    fn decode_engine_with_user_code() {
        let expected = EngineWithUserCode {
            engine_url: "ipfs/Qm".to_string(),
            user_code: "console.log('Hello world')".to_string(),
        };

        let mut injected_bytes = expected.engine_url.as_bytes().to_vec();
        injected_bytes.push(0);
        injected_bytes.extend_from_slice(expected.user_code.as_bytes());
        injected_bytes.push(0);
        injected_bytes.push(0);
        injected_bytes.push(0);

        let result = get_engine_with_user_code(&injected_bytes).unwrap();

        panic!("{:?}", USER_MODULE.len());
        assert_eq!(result, expected);
    }

    // pub async fn load_wrap_from(path: &str) -> (Vec<u8>, Vec<u8>) {
    //     let client = reqwest::Client::new();
    
    //     let manifest_url = format!("{}/wrap.info", path);
    //     let module_url = format!("{}/wrap.wasm", path);
    
    //     let manifest_future = client.get(&manifest_url).send();
    //     let module_future = client.get(&module_url).send();
    
    //     let (manifest_response, module_response) = try_join!(manifest_future, module_future).unwrap();
    
    //     let manifest_bytes = manifest_response.bytes().await.unwrap();
    //     let module_bytes = module_response.bytes().await.unwrap();
    
    //     (manifest_bytes.to_vec(), module_bytes.to_vec())
    // }

    // fn msgpack_to_json_pretty(bytes: &[u8]) -> String {
    //     let value: rmpv::Value = rmp_serde::from_slice(&bytes).unwrap();
    //     serde_json::to_string_pretty(&value).unwrap()
    // }

    // #[tokio::test]
    // async fn download() {
    //     let user_file = "./src/test.js";
    //     let gateway = "https://ipfs.wrappers.io/api/v0/cat?arg=";
    //     let template_wrap_endpoint = format!("{gateway}QmcQ1UorqusoobdroMsSwrg1fnPnRyKLwiEHH2hEvNAfaH");

    //     let user_code = fs::read_to_string(user_file).unwrap();
    //     println!("{}", user_code);

    //     let result = load_wrap_from(&template_wrap_endpoint).await;
    //     let (_manifest, mut module) = result;

    //     replace_user_module(&mut module, &user_code);

    //     let client = get_client_with_module(&module);

    //     let method = "lold";

    //     let result = invoke_client("mock/test", method, &msgpack::msgpack!({
    //         "prop": "arg1"
    //     }), &client);

    //     let result = msgpack_to_json_pretty(&result);

    //     println!("{}", result);

    //     panic!("a");
    // }
}
