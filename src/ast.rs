use syn::visit::{self, Visit};
use syn::{
    Expr, ExprBinary, ExprCall, ExprForLoop, ExprIf, ExprLit, ExprLoop, ExprMatch, ExprMethodCall,
    ExprUnary, ExprWhile, File, ItemFn, Lit, Local, Macro, Member, Pat, Path, ReturnType, Type,
    UnOp,
};

use crate::domain::CompileMode;

#[derive(Debug, Clone, Default)]
pub struct AstFacts {
    let_initializer_count: usize,
    let_initializer_int_values: Vec<u128>,
    let_initializer_paths: Vec<String>,
    let_initializer_call_paths: Vec<String>,
    let_initializer_call_paths_with_int_args: Vec<(String, u128)>,
    let_initializer_deref_paths: Vec<String>,
    let_initializer_if_count: usize,
    struct_pattern_fields: Vec<(String, String)>,
    tuple_pattern_binding_counts: Vec<usize>,
    let_mut_count: usize,
    let_type_names: Vec<String>,
    call_paths: Vec<String>,
    call_paths_with_int_args: Vec<(String, u128)>,
    method_calls: Vec<String>,
    macro_calls: Vec<String>,
    for_loop_count: usize,
    for_loop_by_reference_count: usize,
    while_loop_count: usize,
    while_let_some_count: usize,
    loop_count: usize,
    if_count: usize,
    if_let_some_count: usize,
    match_count: usize,
    function_count: usize,
    function_param_counts: Vec<usize>,
    function_return_type_count: usize,
    array_to_vec_count: usize,
    binary_add_count: usize,
}

#[derive(Debug, Clone)]
pub enum AstAnalysis {
    Parsed(AstFacts),
    ParseError(String),
}

impl AstAnalysis {
    pub fn facts(&self) -> Result<&AstFacts, &str> {
        match self {
            AstAnalysis::Parsed(facts) => Ok(facts),
            AstAnalysis::ParseError(error) => Err(error),
        }
    }
}

impl AstFacts {
    pub fn has_let_mut(&self) -> bool {
        self.let_mut_count > 0
    }

    pub fn has_let_initializer(&self) -> bool {
        self.let_initializer_count > 0
    }

    pub fn has_let_initializer_with_int(&self, value: u128) -> bool {
        self.let_initializer_int_values
            .iter()
            .any(|current| *current == value)
    }

    pub fn has_let_initializer_with_path(&self, path: &str) -> bool {
        self.let_initializer_paths
            .iter()
            .any(|current| current == path)
    }

    pub fn has_let_initializer_with_any_path(&self) -> bool {
        !self.let_initializer_paths.is_empty()
    }

    pub fn has_let_initializer_with_call_path(&self, path: &str) -> bool {
        self.let_initializer_call_paths
            .iter()
            .any(|current| current == path)
    }

    pub fn has_let_initializer_with_call_path_and_int_arg(&self, path: &str, arg: u128) -> bool {
        self.let_initializer_call_paths_with_int_args
            .iter()
            .any(|(current_path, current_arg)| current_path == path && *current_arg == arg)
    }

    pub fn has_let_initializer_with_deref(&self) -> bool {
        !self.let_initializer_deref_paths.is_empty()
    }

    pub fn has_let_initializer_with_deref_path(&self, path: &str) -> bool {
        self.let_initializer_deref_paths
            .iter()
            .any(|current| current == path)
    }

    pub fn has_let_initializer_with_if(&self) -> bool {
        self.let_initializer_if_count > 0
    }

    pub fn has_struct_pattern_field(&self, path: &str, field: &str) -> bool {
        self.struct_pattern_fields
            .iter()
            .any(|(current_path, current_field)| current_path == path && current_field == field)
    }

    pub fn has_tuple_pattern_binding_count(&self, count: usize) -> bool {
        self.tuple_pattern_binding_counts
            .iter()
            .any(|current| *current == count)
    }

    pub fn has_let_type(&self, type_name: &str) -> bool {
        self.let_type_names
            .iter()
            .any(|current| current == type_name)
    }

    pub fn has_call_path(&self, path: &str) -> bool {
        self.call_paths.iter().any(|current| current == path)
    }

    pub fn has_call_path_with_int_arg(&self, path: &str, arg: u128) -> bool {
        self.call_paths_with_int_args
            .iter()
            .any(|(current_path, current_arg)| current_path == path && *current_arg == arg)
    }

    pub fn has_method_call(&self, method: &str) -> bool {
        self.method_call_count(method) > 0
    }

    pub fn method_call_count(&self, method: &str) -> usize {
        self.method_calls
            .iter()
            .filter(|current| current.as_str() == method)
            .count()
    }

    pub fn has_macro_call(&self, macro_name: &str) -> bool {
        self.macro_calls.iter().any(|current| current == macro_name)
    }

    pub fn has_for_loop(&self) -> bool {
        self.for_loop_count > 0
    }

    pub fn has_for_loop_by_reference(&self) -> bool {
        self.for_loop_by_reference_count > 0
    }

    pub fn has_while_loop(&self) -> bool {
        self.while_loop_count > 0
    }

    pub fn has_while_let_some(&self) -> bool {
        self.while_let_some_count > 0
    }

    pub fn has_loop(&self) -> bool {
        self.loop_count > 0
    }

    pub fn has_if(&self) -> bool {
        self.if_count > 0
    }

    pub fn has_if_let_some(&self) -> bool {
        self.if_let_some_count > 0
    }

    pub fn has_match(&self) -> bool {
        self.match_count > 0
    }

    pub fn has_function(&self) -> bool {
        self.function_count > 0
    }

    pub fn has_function_param_count(&self, min: usize) -> bool {
        self.function_param_counts.iter().any(|count| *count >= min)
    }

    pub fn has_function_return_type(&self) -> bool {
        self.function_return_type_count > 0
    }

    pub fn has_array_to_vec(&self) -> bool {
        self.array_to_vec_count > 0
    }

    pub fn has_binary_add(&self) -> bool {
        self.binary_add_count > 0
    }
}

pub fn analyze_answer(answer: &str, mode: CompileMode) -> AstAnalysis {
    let source = match mode {
        CompileMode::Off | CompileMode::FullProgram => answer.to_string(),
        CompileMode::SnippetAsMain => format!("fn main() {{\n{answer}\n}}\n"),
    };

    match syn::parse_file(&source) {
        Ok(file) => AstAnalysis::Parsed(collect_facts(&file)),
        Err(error) => AstAnalysis::ParseError(error.to_string()),
    }
}

fn collect_facts(file: &File) -> AstFacts {
    let mut visitor = FactVisitor::default();
    visitor.visit_file(file);
    visitor.facts
}

#[derive(Default)]
struct FactVisitor {
    facts: AstFacts,
}

impl<'ast> Visit<'ast> for FactVisitor {
    fn visit_local(&mut self, local: &'ast Local) {
        collect_local_initializer(local, &mut self.facts);
        collect_struct_pattern_fields(&local.pat, &mut self.facts.struct_pattern_fields);
        collect_tuple_pattern_binding_counts(
            &local.pat,
            &mut self.facts.tuple_pattern_binding_counts,
        );

        if pat_has_mut_binding(&local.pat) {
            self.facts.let_mut_count += 1;
        }

        collect_pat_types(&local.pat, &mut self.facts.let_type_names);

        visit::visit_local(self, local);
    }

    fn visit_expr_call(&mut self, call: &'ast ExprCall) {
        if let Expr::Path(path) = call.func.as_ref() {
            let call_path = path_to_string(&path.path);

            if call_path == "Vec::from" && call.args.iter().any(|arg| matches!(arg, Expr::Array(_)))
            {
                self.facts.array_to_vec_count += 1;
            }

            for arg in &call.args {
                if let Some(value) = expr_int_literal(arg) {
                    self.facts
                        .call_paths_with_int_args
                        .push((call_path.clone(), value));
                }
            }

            self.facts.call_paths.push(call_path);
        }

        visit::visit_expr_call(self, call);
    }

    fn visit_expr_method_call(&mut self, call: &'ast ExprMethodCall) {
        let method = call.method.to_string();

        if method == "to_vec" {
            self.facts.array_to_vec_count += 1;
        }

        self.facts.method_calls.push(method);
        visit::visit_expr_method_call(self, call);
    }

    fn visit_macro(&mut self, macro_call: &'ast Macro) {
        if let Some(segment) = macro_call.path.segments.last() {
            self.facts.macro_calls.push(segment.ident.to_string());
        }

        visit::visit_macro(self, macro_call);
    }

    fn visit_expr_for_loop(&mut self, for_loop: &'ast ExprForLoop) {
        self.facts.for_loop_count += 1;

        if matches!(for_loop.expr.as_ref(), Expr::Reference(_)) {
            self.facts.for_loop_by_reference_count += 1;
        }

        visit::visit_expr_for_loop(self, for_loop);
    }

    fn visit_expr_while(&mut self, while_loop: &'ast ExprWhile) {
        self.facts.while_loop_count += 1;

        if expr_is_let_some(while_loop.cond.as_ref()) {
            self.facts.while_let_some_count += 1;
        }

        visit::visit_expr_while(self, while_loop);
    }

    fn visit_expr_loop(&mut self, loop_expr: &'ast ExprLoop) {
        self.facts.loop_count += 1;
        visit::visit_expr_loop(self, loop_expr);
    }

    fn visit_expr_if(&mut self, if_expr: &'ast ExprIf) {
        self.facts.if_count += 1;

        if expr_is_let_some(if_expr.cond.as_ref()) {
            self.facts.if_let_some_count += 1;
        }

        visit::visit_expr_if(self, if_expr);
    }

    fn visit_expr_match(&mut self, match_expr: &'ast ExprMatch) {
        self.facts.match_count += 1;
        visit::visit_expr_match(self, match_expr);
    }

    fn visit_item_fn(&mut self, item_fn: &'ast ItemFn) {
        self.facts.function_count += 1;
        self.facts
            .function_param_counts
            .push(item_fn.sig.inputs.len());

        if !matches!(item_fn.sig.output, ReturnType::Default) {
            self.facts.function_return_type_count += 1;
        }

        visit::visit_item_fn(self, item_fn);
    }

    fn visit_expr_binary(&mut self, binary: &'ast ExprBinary) {
        if matches!(binary.op, syn::BinOp::Add(_)) {
            self.facts.binary_add_count += 1;
        }

        visit::visit_expr_binary(self, binary);
    }
}

fn collect_local_initializer(local: &Local, facts: &mut AstFacts) {
    if !pat_is_simple_ident(&local.pat) {
        return;
    }

    let Some(init) = &local.init else {
        return;
    };

    facts.let_initializer_count += 1;
    let expr = init.expr.as_ref();

    if let Some(value) = expr_int_literal(expr) {
        facts.let_initializer_int_values.push(value);
    }

    match expr {
        Expr::Path(path) => facts.let_initializer_paths.push(path_to_string(&path.path)),
        Expr::Call(call) => collect_let_call_initializer(call, facts),
        Expr::Unary(unary) => collect_let_unary_initializer(unary, facts),
        Expr::If(_) => facts.let_initializer_if_count += 1,
        _ => {}
    }
}

fn collect_let_call_initializer(call: &ExprCall, facts: &mut AstFacts) {
    let Expr::Path(path) = call.func.as_ref() else {
        return;
    };

    let call_path = path_to_string(&path.path);
    facts.let_initializer_call_paths.push(call_path.clone());

    for arg in &call.args {
        if let Some(value) = expr_int_literal(arg) {
            facts
                .let_initializer_call_paths_with_int_args
                .push((call_path.clone(), value));
        }
    }
}

fn collect_let_unary_initializer(unary: &ExprUnary, facts: &mut AstFacts) {
    if !matches!(unary.op, UnOp::Deref(_)) {
        return;
    }

    if let Expr::Path(path) = unary.expr.as_ref() {
        facts
            .let_initializer_deref_paths
            .push(path_to_string(&path.path));
    }
}

fn pat_is_simple_ident(pat: &Pat) -> bool {
    match pat {
        Pat::Ident(ident) => ident.ident != "_",
        Pat::Type(typed) => pat_is_simple_ident(&typed.pat),
        _ => false,
    }
}

fn collect_struct_pattern_fields(pat: &Pat, fields: &mut Vec<(String, String)>) {
    match pat {
        Pat::Struct(struct_pat) => {
            let path = path_to_string(&struct_pat.path);
            for field in &struct_pat.fields {
                if let Member::Named(name) = &field.member {
                    fields.push((path.clone(), name.to_string()));
                }
                collect_struct_pattern_fields(&field.pat, fields);
            }
        }
        Pat::Type(typed) => collect_struct_pattern_fields(&typed.pat, fields),
        Pat::Tuple(tuple) => {
            for elem in &tuple.elems {
                collect_struct_pattern_fields(elem, fields);
            }
        }
        Pat::TupleStruct(tuple_struct) => {
            for elem in &tuple_struct.elems {
                collect_struct_pattern_fields(elem, fields);
            }
        }
        Pat::Reference(reference) => collect_struct_pattern_fields(&reference.pat, fields),
        Pat::Slice(slice) => {
            for elem in &slice.elems {
                collect_struct_pattern_fields(elem, fields);
            }
        }
        _ => {}
    }
}

fn collect_tuple_pattern_binding_counts(pat: &Pat, counts: &mut Vec<usize>) {
    match pat {
        Pat::Tuple(tuple) => {
            if !tuple.elems.is_empty() && tuple.elems.iter().all(pat_is_simple_ident) {
                counts.push(tuple.elems.len());
            }

            for elem in &tuple.elems {
                collect_tuple_pattern_binding_counts(elem, counts);
            }
        }
        Pat::Type(typed) => collect_tuple_pattern_binding_counts(&typed.pat, counts),
        Pat::Reference(reference) => collect_tuple_pattern_binding_counts(&reference.pat, counts),
        Pat::Slice(slice) => {
            for elem in &slice.elems {
                collect_tuple_pattern_binding_counts(elem, counts);
            }
        }
        _ => {}
    }
}

fn pat_has_mut_binding(pat: &Pat) -> bool {
    match pat {
        Pat::Ident(ident) => ident.mutability.is_some(),
        Pat::Type(typed) => pat_has_mut_binding(&typed.pat),
        Pat::Tuple(tuple) => tuple.elems.iter().any(pat_has_mut_binding),
        Pat::TupleStruct(tuple_struct) => tuple_struct.elems.iter().any(pat_has_mut_binding),
        Pat::Struct(struct_pat) => struct_pat
            .fields
            .iter()
            .any(|field| pat_has_mut_binding(&field.pat)),
        Pat::Reference(reference) => pat_has_mut_binding(&reference.pat),
        Pat::Slice(slice) => slice.elems.iter().any(pat_has_mut_binding),
        _ => false,
    }
}

fn expr_is_let_some(expr: &Expr) -> bool {
    let Expr::Let(expr_let) = expr else {
        return false;
    };

    pat_is_some_tuple(&expr_let.pat)
}

fn pat_is_some_tuple(pat: &Pat) -> bool {
    match pat {
        Pat::TupleStruct(tuple_struct) => path_to_string(&tuple_struct.path) == "Some",
        Pat::Type(typed) => pat_is_some_tuple(&typed.pat),
        _ => false,
    }
}

fn collect_pat_types(pat: &Pat, type_names: &mut Vec<String>) {
    match pat {
        Pat::Type(typed) => {
            collect_type_names(&typed.ty, type_names);
            collect_pat_types(&typed.pat, type_names);
        }
        Pat::Tuple(tuple) => {
            for elem in &tuple.elems {
                collect_pat_types(elem, type_names);
            }
        }
        Pat::TupleStruct(tuple_struct) => {
            for elem in &tuple_struct.elems {
                collect_pat_types(elem, type_names);
            }
        }
        Pat::Struct(struct_pat) => {
            for field in &struct_pat.fields {
                collect_pat_types(&field.pat, type_names);
            }
        }
        Pat::Reference(reference) => collect_pat_types(&reference.pat, type_names),
        Pat::Slice(slice) => {
            for elem in &slice.elems {
                collect_pat_types(elem, type_names);
            }
        }
        _ => {}
    }
}

fn collect_type_names(ty: &Type, type_names: &mut Vec<String>) {
    match ty {
        Type::Path(path) => {
            for segment in &path.path.segments {
                type_names.push(segment.ident.to_string());
            }
        }
        Type::Reference(reference) => collect_type_names(&reference.elem, type_names),
        Type::Slice(slice) => collect_type_names(&slice.elem, type_names),
        Type::Array(array) => collect_type_names(&array.elem, type_names),
        Type::Tuple(tuple) => {
            for elem in &tuple.elems {
                collect_type_names(elem, type_names);
            }
        }
        _ => {}
    }
}

fn expr_int_literal(expr: &Expr) -> Option<u128> {
    match expr {
        Expr::Lit(ExprLit {
            lit: Lit::Int(value),
            ..
        }) => value.base10_parse::<u128>().ok(),
        _ => None,
    }
}

fn path_to_string(path: &Path) -> String {
    path.segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect::<Vec<_>>()
        .join("::")
}
