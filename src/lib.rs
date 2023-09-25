/*
 * Copyright 2023 Amazon.com, Inc. or its affiliates.  All rights reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License").
 *  You may not use this file except in compliance with the License.
 * A copy of the License is located at:
 *
 *      http://aws.amazon.com/apache2.0/
 *
 *  or in the "license" file accompanying this file. This file is distributed on an "AS IS" BASIS,
 *  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific
 *  language governing permissions and limitations under the License.
 */

use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;

use partiql_catalog::{Catalog, PartiqlCatalog};
use partiql_eval::env::basic::MapBindings;
use partiql_eval::eval::EvalPlan;
use partiql_eval::plan::{EvaluationMode, EvaluatorPlanner};
use partiql_parser::Parser;
use partiql_value::Value;

use partiql_extension_ion::decode::{IonDecodeResult, IonDecoderBuilder, IonDecoderConfig};
use partiql_extension_ion::Encoding;
use partiql_logical::{BindingsOp, LogicalPlan};

/// Provides a Python binding for evaluating the given query using the given environment:
/// See https://github.com/partiql/partiql-rust-py/README.md#Development for an example
/// usage in Python.
#[pyfunction]
fn eval(query: &str, env: &str) -> PyResult<String> {
    let bindings = to_bindings(env);
    // TODO Allow catalog as an input
    let catalog = PartiqlCatalog::default();
    let mut compiler = init_compiler(query, &catalog)?;

    if let Ok(out) = compiler.execute_mut(bindings) {
        let res = &out.result;
        Ok(format!("{:?}", &res))
    } else {
        Err(PyErr::new::<PyTypeError, _>("Evaluation Error"))
    }
}

fn to_bindings(env: &str) -> MapBindings<Value> {
    let env_as_ion = decode_ion_text(env, Encoding::Ion).unwrap_or(Value::Missing);
    MapBindings::from(env_as_ion)
}

fn init_compiler(query: &str, catalog: &dyn Catalog) -> Result<EvalPlan, PyErr> {
    let lowered = lower(query, catalog)?;
    let mut compiler = EvaluatorPlanner::new(EvaluationMode::Permissive, catalog);
    compiler
        .compile(&lowered)
        .map_err(|err| PyTypeError::new_err(format!("{:?}", err)))
}

fn lower(query: &str, catalog: &dyn Catalog) -> Result<LogicalPlan<BindingsOp>, PyErr> {
    let planner = partiql_logical_planner::LogicalPlanner::new(catalog);
    let parsed = Parser::default()
        .parse(query)
        .map_err(|err| PyTypeError::new_err(format!("{:?}", err)))?;
    planner
        .lower(&parsed)
        .map_err(|err| PyTypeError::new_err(format!("{:?}", err)))
}

fn decode_ion_text(contents: &str, encoding: Encoding) -> IonDecodeResult {
    let reader = ion_rs::ReaderBuilder::new().build(contents)?;
    let mut iter =
        IonDecoderBuilder::new(IonDecoderConfig::default().with_mode(encoding)).build(reader)?;

    let val = iter.next();

    val.unwrap()
}

/// A Python module implemented in Rust.
#[pymodule]
fn partiql_rust_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(eval, m)?)?;
    Ok(())
}
