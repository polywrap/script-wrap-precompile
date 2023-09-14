use std::sync::Arc;

use polywrap_client::{client::PolywrapClient, wasm::wasm_wrapper::WasmWrapper, core::{uri::Uri, file_reader::SimpleFileReader}, builder::{PolywrapClientConfig, PolywrapClientConfigBuilder}};
use polywrap_client_default_config::SystemClientConfig;

pub fn get_client_with_module(module: &[u8]) -> PolywrapClient {
    let mut config = {
        PolywrapClientConfig {
            interfaces: None,
            envs: None,
            wrappers: Some(vec![
                (
                    Uri::try_from("wrap://mock/test").unwrap(),
                    Arc::new(WasmWrapper::try_from_bytecode(module, Arc::new(SimpleFileReader::new())).unwrap()),
                ),
                // (
                //   Uri::try_from("wrap://mock/engine").unwrap(),
                //   Arc::new(WasmWrapper::new(load_wrap("./py-engine").1.to_vec(), Arc::new(SimpleFileReader::new()))),
                // ),
            ]),
            packages: None,
            // redirects: Some(vec![
            //     //   UriRedirect::new(Uri::try_from("wrap://ipfs/QmUhxYCRG3C7siCydRUqXqz7FZWCMx1kfmMcAMZq52WTzU").unwrap(), Uri::try_from("wrap://mock/engine").unwrap()),
            // ]),
            resolvers: None,
            ..Default::default()
        }
    };
    config.add(SystemClientConfig::default().into());

    let client = PolywrapClient::new(config.into());

    client
}
