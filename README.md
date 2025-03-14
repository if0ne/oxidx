# oxidx

This project provides low-level wrapper for D3D12 API.

## Features

* Provides a low-level API around DirectX 12. All methods correspond to DirectX 12 C++ methods, but they are done in a Rust way.
* Based on official windows [crate](https://github.com/microsoft/windows-rs).
* No library/runtime validation, only driver validation.
* PIX methods.
* D3D12 and DXGI prefixes have been stripped from all types.

## Minimum supported Rust version

oxidx's MSRV is 1.80.

## Examples

### Device creation

```rust
let factory = create_factory4(FactoryCreationFlags::empty())?;

let adapter = factory.enum_adapters(0)?;

let device = create_device(&adapter, FeatureLevel::Level11)?;
```

### Feature fetching

```rust
let mut options = Options1Feature::default();
device.check_feature_support(&mut options)?;
```
