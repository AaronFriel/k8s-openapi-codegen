[![Build Status](https://dev.azure.com/arnavion/k8s-openapi-codegen/_apis/build/status/Arnavion.k8s-openapi-codegen?branchName=master)](https://dev.azure.com/arnavion/k8s-openapi-codegen/_build/latest?definitionId=1)

This binary generates Rust types for the resources and API in the Kubernetes OpenAPI spec.


# Generating the bindings

Run this binary:

```sh
cargo run
```

Bindings will now be generated in the `k8s-openapi/` directory.


# Using the bindings

See `k8s-openapi/README.md` for information about the bindings crate itself.

See the `k8s-openapi-tests/` directory for examples.
