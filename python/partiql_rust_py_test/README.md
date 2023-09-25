# partiql_rust_py_test
This Python module provides APIs for integration testing.

## Example Usage

```python
import partiql_rust_py_test
partiql_rust_py_test.eval_test("SELECT t.a + 1 FROM MyTable As t", "{'MyTable': {'a': 1}}")
```
