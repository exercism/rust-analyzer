pub mod comments;
mod lints;
#[cfg(test)]
mod test;

use crate::{
    analyzers::{
        output::{AnalysisOutput, AnalysisStatus},
        Analyze,
    },
    Result,
};
use comments::ReverseStringComment;
use lints::REVERSE_STRING_LINTS;
use syn::File;

pub struct ReverseStringAnalyzer;

/// Checks if the solution AST contains the necessary function
/// definition. For `reverse-string` this means checking if the
/// following function is present in the AST:
///
/// pub fn reverse(input: &str) -> String {}
fn solution_contains_neccesary_fn(ast: &File) -> bool {
    let solution_fn = if let Some(syn::Item::Fn(func)) = ast.items.iter().find(|item| {
        if let syn::Item::Fn(item_fn) = item {
            item_fn.sig.ident == "reverse"
        } else {
            false
        }
    }) {
        func
    } else {
        return false;
    };
    let solution_fn_is_public = matches!(solution_fn.vis, syn::Visibility::Public(_));

    let solution_fn_returns_string =
        if let syn::ReturnType::Type(_, ref return_type) = solution_fn.sig.output {
            if let syn::Type::Path(return_type_path) = return_type.as_ref() {
                let segments = &return_type_path.path.segments;
                segments.len() == 1
                    && if let Some(segment) = segments.first() {
                        segment.ident == "String"
                    } else {
                        false
                    }
            } else {
                false
            }
        } else {
            false
        };
    let solution_fn_accepts_str =
        if let Some(syn::FnArg::Typed(syn::PatType { ty, .. })) = solution_fn.sig.inputs.first() {
            if let syn::Type::Reference(ref typ) = **ty {
                if let syn::Type::Path(syn::TypePath {
                    path: syn::Path { segments, .. },
                    ..
                }) = typ.elem.as_ref()
                {
                    if let Some(syn::PathSegment { ident, .. }) = segments.first() {
                        ident == "str"
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        } && solution_fn.sig.inputs.len() == 1;
    solution_fn_is_public && solution_fn_returns_string && solution_fn_accepts_str
}

impl Analyze for ReverseStringAnalyzer {
    fn analyze(&self, solution_ast: &File, _solution_raw: &str) -> Result<AnalysisOutput> {
        // Check if the 'reverse' function is present, before
        // running any lints on the solution.
        if !solution_contains_neccesary_fn(solution_ast) {
            return Ok(AnalysisOutput::new(
                AnalysisStatus::Disapprove,
                vec![ReverseStringComment::SolutionFunctionNotFound.to_string()],
            ));
        }
        let mut analysis_output = AnalysisOutput::new(AnalysisStatus::ReferToMentor, vec![]);
        REVERSE_STRING_LINTS
            .iter()
            .try_for_each(|lint| lint(solution_ast, &mut analysis_output))
            .map(|_| analysis_output)
    }
}
