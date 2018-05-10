const wasm = import('./lib.rs')

wasm.then(rust => {
  rust.greet('J. Driller')
});