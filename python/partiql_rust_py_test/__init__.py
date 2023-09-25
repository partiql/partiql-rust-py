from partiql_rust_py import eval

def eval_test(query: str, env: str):
    """
    Returns the query evaluation results for the given `query` and `env`

    Arguments:
        `query` -- A PartiQL query, e.g., `SELECT t.a + 1 FROM {'a': 1} As t`
        `env` -- Evaluation Environment, e.g., "{'MyTable': {'a': 1}}"
    Returns:
    Query Evaluation Result. Examples:
    >>> partiql_rust_py_test.eval_test('1 + 1', '{}')
    '2'

    >>> partiql_rust_py_test.eval_test("SELECT t.a + 1 FROM MyTable As t", "{'MyTable': {'a': 1}}")
    '<<{ _1: 2 }>>'
    """
    return eval(query, env)