# PartiQL Rust Py

## About
**_Note: This project is experimental and the APIs that it provides are unstable._**

PartiQL Rust Python is a [Rust](https://www.rust-lang.org/) bindings package for [Python](https://www.python.org/).
It provides PartiQL language APIs to python consumers using [partiql-lang-rust](https://github.com/partiql/partiql-lang-rust) APIs.

`partiql-rust-py` uses [PyO3](https://github.com/PyO3/pyo3) to provide a Python module in Rust to expose an API that uses PartiQL Rust Evaluator to evaluate input PartiQL queries in Python.

## License
This project is licensed under the Apache-2.0 License.

## Security
See [CONTRIBUTING](CONTRIBUTING.md#security-issue-notifications) for more information.

## Development

First, in project's directory follow the commands below to create a Python `virtualenv`, and install `maturin` into the 
virtualenv using Python's package manager, pip:
```bash
python -m venv .env && \
  source .env/bin/activate && \
  pip install maturin
```

Once the environment is set up, after making changes to the project source run `maturin develop` to rebuild the bindings.

## Testing 

### Via Python REPL

```bash
source .env/bin/activate && python
Python 3.11.4 (main, Jun 20 2023, 17:37:48) [Clang 14.0.0 (clang-1400.0.29.202)] on darwin
Type "help", "copyright", "credits" or "license" for more information.

>>> import partiql_rust_py
>>> partiql_rust_py.eval("1 + 1", "{}")
'2'
>>> 
```

### Via `python/partiql_rust_py_test`
This Python module provides APIs for integration testing:

```python
import partiql_rust_py_test
partiql_rust_py_test.eval_test("SELECT t.a + 1 FROM MyTable As t", "{'MyTable': {'a': 1}}")
```

## References

1. PyO3: https://github.com/PyO3/pyo3