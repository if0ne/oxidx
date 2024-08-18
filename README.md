# oxidx

This project provides low-level bindings/wrapper for D3D12 API.

## Features

* Provides a low-level API around DirectX 12. All methods correspond to DirectX 12 C++ methods, but they are done in a Rust way.
* Based on official windows [crate](https://github.com/microsoft/windows-rs).
* No library/runtime validation, only driver validation.
* PIX methods.
* D3D12 and DXGI prefixes have been stripped from all types.

## Examples

### Device creation

```rust
let entry = Entry;

let factory: Factory4 = entry.create_factory(FactoryCreationFlags::empty())?;

let adapter = factory.enum_adapters(0)?;

let device: Device = entry
    .create_device(&adapter, FeatureLevel::Level11)
    .unwrap();
```

### Feature fetching

```rust
let feature = device.check_feature_support::<Options>(())?;
```
