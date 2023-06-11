mod wrap;
use wrap::*;
use wrap::JsEngineModule;

const BOILERPLATE: &str = include_str!("boilerplate.js");
const USER_MODULE: &[u8; 0] = include_bytes!("index.js");

pub fn run_js_wrap(method: &str, args: &[u8]) -> Vec<u8> {
    _run_js_wrap(method, args, &|| {
        String::from_utf8(bytes_up_to_zero(USER_MODULE).to_vec()).unwrap()
    }, &|| {
        String::from(BOILERPLATE)
    })
}

fn _run_js_wrap(method: &str, args: &[u8], load_user_module: &dyn Fn() -> String, load_boilerplate: & dyn Fn() -> String) -> Vec<u8> {
    let json = msgpack_to_json(args);

    let extern_code = load_user_module();
    let boilerplate = load_boilerplate();
    
    let call = format!("{method}(JSON.parse('{json}'));");
    let args = wrap::imported::ArgsEval {
        src: format!("{boilerplate}\n\n{extern_code}\n\n{call}"),
    };

    let result = JsEngineModule::eval(&args);

    let result: JsEngineEvalResult = result.unwrap();

    if let Some(error) = result.error {
        panic!("{}", error);
    };

    let result = result.value.unwrap();

    let result = json_to_msgpack(&result.to_string());

    return result;
}

fn msgpack_to_json(bytes: &[u8]) -> String {
    let value: rmpv::Value = rmp_serde::from_slice(&bytes).unwrap();
    serde_json::to_string(&value).unwrap()
}

fn json_to_msgpack(string: &str) -> Vec<u8> {
    let value: serde_json::Value = serde_json::from_str(string).unwrap();
    rmp_serde::encode::to_vec(&value).unwrap()
}

fn bytes_up_to_zero(slice: &[u8]) -> &[u8] {
    match slice.iter().position(|&x| x == 0) {
        Some(index) => &slice[..index],
        None => slice,
    }
}

#[cfg(test)]
mod tests {
}
