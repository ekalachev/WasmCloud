use std::error::Error;
use std::io::Read;

use wasmer::{Module, Store};
use wasmer_wasix::{Pipe, WasiEnv};
use isahc;

/// This function downloads the wasm binary from IPFS, executes it,
/// and returns the string output of the wasm execution.
fn execute_wasm(name: String) -> Result<String, Box<dyn Error>> {
    // The IPFS URI to the wasm binary
    let uri = format!("https://ipfs.io/ipfs/QmUcuakCUoTCQQ28Dx3yToziRhPuWAeh11K1btKuYzyWSF?filename=HelloWorld.wasm");

    // Download the wasm binary from IPFS
    let response_bytes = download_from_ipfs(uri)?;

    // Execute the wasm binary and get its output

    // Create a new wasm store
    let mut store = Store::default();

    // Create a new wasm module from the binary
    let module = Module::new(&store, &response_bytes)?;

    // Create a channel for the wasm module's stdout
    let (stdout_tx, mut stdout_rx) = Pipe::channel();

    // Build the wasm environment and execute the module
    let mut builder = WasiEnv::builder("hello")
        .stdout(Box::new(stdout_tx));

    if !name.is_empty() {
        builder = builder.env("NAME", name);
    }

    builder.run_with_store(module, &mut store)?;

    // Read the output of the wasm module
    let mut buf = String::new();
    stdout_rx.read_to_string(&mut buf).unwrap();

    Ok(buf)
}

/// This function downloads a wasm binary from IPFS
fn download_from_ipfs(uri: String) -> Result<Vec<u8>, Box<dyn Error>> {
    // Send a GET request to the IPFS URI
    let response_result = isahc::get(uri)?;

    // Read the entire response into a byte vector
    let mut response_bytes = Vec::new();
    response_result.into_body().read_to_end(&mut response_bytes)?;

    Ok(response_bytes)
}

/// This is the main function. It executes the wasm binary and prints its output.
fn main() -> Result<(), Box<dyn Error>> {
    // Execute the wasm binary and get its output
    let output = execute_wasm("Eldar".to_string())?;

    // Print the output
    println!("{}", output);

    Ok(())
}

/// This is the test module. It tests the execute_wasm function.
#[cfg(test)]
mod test {
    use crate::{execute_wasm, download_from_ipfs, main};

    #[test]
    fn can_handle_request() {
        let response = execute_wasm("Tom".to_string()).unwrap();
        assert_eq!(
            response,
            "Hello, Tom!\n".to_string()
        );
    }

    #[test]
    fn default_can_handle_request() {
        let response = execute_wasm("".to_string()).unwrap();
        assert_eq!(
            response,
            "Hello, World!\n".to_string()
        );
    }

    #[test]
    fn test_download_from_ipfs() {
        let uri = format!("https://ipfs.io/ipfs/QmUcuakCUoTCQQ28Dx3yToziRhPuWAeh11K1btKuYzyWSF?filename=HelloWorld.wasm");
        let response_bytes = download_from_ipfs(uri).unwrap();
        assert!(!response_bytes.is_empty());
    }

    #[test]
    fn test_execute_wasm() {
        assert!(execute_wasm("".to_string()).is_ok());
    }

    #[test]
    fn test_main() {
        assert!(main().is_ok());
    }
}

/// This test runs the main function when the "wasi" feature is enabled
#[test]
#[cfg(feature = "wasi")]
fn test_wasi() -> Result<(), Box<dyn std::error::Error>> {
    main()
}
