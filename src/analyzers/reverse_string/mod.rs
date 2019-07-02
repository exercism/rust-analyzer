pub mod comments;
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
use syn::File;

pub struct ReverseStringAnalyzer;

type ReverseStringLint = Box<dyn Fn(&File, &mut AnalysisOutput) -> Result<()>>;

/// Generates a vector of the linting functions. Each one of them accepts a reference
/// to the solution AST and a mutable reference to the analysis output and
/// modifies the output in accordance to the lint that they represent.
fn generate_lints() -> Vec<ReverseStringLint> {
    vec![Box::new(|ast, output| {
        if ast.items.iter().any(|item| {
            if let syn::Item::ExternCrate(_) = item {
                true
            } else {
                false
            }
        }) {
            output
                .comments
                .push(ReverseStringComment::SuggestRemovingExternCrate.to_string())
        }
        Ok(())
    })]
}

/// Checks if the solution AST contains the necessary function
/// definition. For `reverse-string` this means checking if the
/// following function is present in the AST:
///
/// pub fn reverse(input: &str) -> String {}
fn solution_contains_neccesary_fn(ast: &File) -> bool {
    let solution_fn = if let Some(syn::Item::Fn(func)) = ast.items.iter().find(|item| {
        if let syn::Item::Fn(item_fn) = item {
            item_fn.ident == "reverse"
        } else {
            false
        }
    }) {
        func
    } else {
        return false;
    };
    let solution_fn_is_public = if let syn::Visibility::Public(_) = solution_fn.vis {
        true
    } else {
        false
    };
    let solution_fn_returns_string =
        if let syn::ReturnType::Type(_, ref return_type) = solution_fn.decl.output {
            if let syn::Type::Path(return_type_path) = return_type.as_ref() {
                let segments = &return_type_path.path.segments;
                segments.len() == 1
                    && if let Some(segment) = segments.first() {
                        segment.value().ident == "String"
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
        if let Some(syn::punctuated::Pair::End(syn::FnArg::Captured(syn::ArgCaptured {
            ty: syn::Type::Reference(typ),
            ..
        }))) = solution_fn.decl.inputs.first()
        {
            if let syn::Type::Path(syn::TypePath {
                path: syn::Path { segments, .. },
                ..
            }) = typ.elem.as_ref()
            {
                if let Some(syn::punctuated::Pair::End(syn::PathSegment { ident, .. })) =
                    segments.first()
                {
                    ident == "str"
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        } && solution_fn.decl.inputs.len() == 1;
    solution_fn_is_public && solution_fn_returns_string && solution_fn_accepts_str
}

impl Analyze for ReverseStringAnalyzer {
    fn analyze(&self, solution_ast: &File) -> Result<AnalysisOutput> {
        // Check if the 'reverse' function is present, before
        // running any lints on the solution.
        if !solution_contains_neccesary_fn(solution_ast) {
            return Ok(AnalysisOutput::new(
                AnalysisStatus::Disapprove,
                vec![ReverseStringComment::SolutionFunctionNotFound.to_string()],
            ));
        }
        let mut analysis_output = AnalysisOutput::new(AnalysisStatus::ReferToMentor, vec![]);
        generate_lints()
            .iter()
            .try_for_each(|lint| lint(solution_ast, &mut analysis_output))
            .map(|_| analysis_output)
    }
}
