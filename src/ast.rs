use syn::visit::{self, Visit};
use syn::{
    Expr, ExprBinary, ExprCall, ExprForLoop, ExprIf, ExprLit, ExprLoop, ExprMatch, ExprMethodCall,
    ExprWhile, File, ItemFn, Lit, Local, Macro, Pat, Path, ReturnType, Type,
};

use crate::domain::CompileMode;

#[derive(Debug, Clone, Default)]
pub struct AstFacts {
    let_mut_count: usize,
    let_type_names: Vec<String>,
    call_paths: Vec<String>,
    call_paths_with_int_args: Vec<(String, u128)>,
    method_calls: Vec<String>,
    macro_calls: Vec<String>,
    for_loop_count: usize,
    for_loop_by_reference_count: usize,
    while_loop_count: usize,
    loop_count: usize,
    if_count: usize,
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

    pub fn has_loop(&self) -> bool {
        self.loop_count > 0
    }

    pub fn has_if(&self) -> bool {
        self.if_count > 0
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
        visit::visit_expr_while(self, while_loop);
    }

    fn visit_expr_loop(&mut self, loop_expr: &'ast ExprLoop) {
        self.facts.loop_count += 1;
        visit::visit_expr_loop(self, loop_expr);
    }

    fn visit_expr_if(&mut self, if_expr: &'ast ExprIf) {
        self.facts.if_count += 1;
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
