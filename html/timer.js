const rust = import('../pkg/druid_wasm_examples');

rust
  .then(m => m.timer())
  .catch(console.error);
