// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

//! "core" DataFusion functions

use datafusion_expr::ScalarUDF;
use std::sync::Arc;

pub mod arrow_cast;
pub mod arrowtypeof;
pub mod coalesce;
pub mod expr_ext;
pub mod getfield;
pub mod greatest;
mod greatest_least_utils;
pub mod least;
pub mod named_struct;
pub mod nullif;
pub mod nvl;
pub mod nvl2;
pub mod overlay;
pub mod planner;
pub mod r#struct;
pub mod union_extract;
pub mod union_tag;
pub mod version;

// create UDFs
make_udf_function!(arrow_cast::ArrowCastFunc, arrow_cast);
make_udf_function!(nullif::NullIfFunc, nullif);
make_udf_function!(nvl::NVLFunc, nvl);
make_udf_function!(nvl2::NVL2Func, nvl2);
make_udf_function!(overlay::OverlayFunc, overlay);
make_udf_function!(arrowtypeof::ArrowTypeOfFunc, arrow_typeof);
make_udf_function!(r#struct::StructFunc, r#struct);
make_udf_function!(named_struct::NamedStructFunc, named_struct);
make_udf_function!(getfield::GetFieldFunc, get_field);
make_udf_function!(coalesce::CoalesceFunc, coalesce);
make_udf_function!(greatest::GreatestFunc, greatest);
make_udf_function!(least::LeastFunc, least);
make_udf_function!(union_extract::UnionExtractFun, union_extract);
make_udf_function!(union_tag::UnionTagFunc, union_tag);
make_udf_function!(version::VersionFunc, version);

pub mod expr_fn {
    use datafusion_expr::{Expr, Literal};

    export_functions!((
        nullif,
        "Returns NULL if value1 equals value2; otherwise it returns value1. This can be used to perform the inverse operation of the COALESCE expression",
        arg1 arg2
    ),(
        arrow_cast,
        "Returns value2 if value1 is NULL; otherwise it returns value1",
        arg1 arg2
    ),(
        nvl,
        "Returns value2 if value1 is NULL; otherwise it returns value1",
        arg1 arg2
    ),(
        nvl2,
        "Returns value2 if value1 is not NULL; otherwise, it returns value3.",
        arg1 arg2 arg3
    ),(
        overlay,
        "replace the substring of string that starts at the start'th character and extends for count characters with new substring",
        args,
    ),(
        arrow_typeof,
        "Returns the Arrow type of the input expression.",
        arg1
    ),(
        r#struct,
        "Returns a struct with the given arguments",
        args,
    ),(
        named_struct,
        "Returns a struct with the given names and arguments pairs",
        args,
    ),(
        coalesce,
        "Returns `coalesce(args...)`, which evaluates to the value of the first expr which is not NULL",
        args,
    ),(
        greatest,
        "Returns `greatest(args...)`, which evaluates to the greatest value in the list of expressions or NULL if all the expressions are NULL",
        args,
    ),(
        least,
        "Returns `least(args...)`, which evaluates to the smallest value in the list of expressions or NULL if all the expressions are NULL",
        args,
    ),(
        union_tag,
        "Returns the name of the currently selected field in the union",
        arg1
    ));

    #[doc = "Returns the value of the field with the given name from the struct"]
    pub fn get_field(arg1: Expr, arg2: impl Literal) -> Expr {
        super::get_field().call(vec![arg1, arg2.lit()])
    }

    #[doc = "Returns the value of the field with the given name from the union when it's selected, or NULL otherwise"]
    pub fn union_extract(arg1: Expr, arg2: impl Literal) -> Expr {
        super::union_extract().call(vec![arg1, arg2.lit()])
    }
}

/// Returns all DataFusion functions defined in this package
pub fn functions() -> Vec<Arc<ScalarUDF>> {
    vec![
        nullif(),
        arrow_cast(),
        nvl(),
        nvl2(),
        overlay(),
        arrow_typeof(),
        named_struct(),
        // Note: most users invoke `get_field` indirectly via field access
        // syntax like `my_struct_col['field_name']`, which results in a call to
        // `get_field(my_struct_col, "field_name")`.
        //
        // However, it is also exposed directly for use cases such as
        // serializing / deserializing plans with the field access desugared to
        // calls to [`get_field`]
        get_field(),
        coalesce(),
        greatest(),
        least(),
        union_extract(),
        union_tag(),
        version(),
        r#struct(),
    ]
}
