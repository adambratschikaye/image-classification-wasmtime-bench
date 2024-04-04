# Image Classification Wasmtime Benchmark

A benchmark that runs a simple image classification library in Wasmtime.
Wasmtime is run with optimizations disabled and NaN canonicalization enabled.

Note that the classifier model is not included in the repo because it is large
and is instead downloaded if needed when running the `run.sh` script.