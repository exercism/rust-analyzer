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
use lazy_static::lazy_static;
use syn::File;

pub struct ReverseStringAnalyzer;

struct PreparedOutput<'a> {
    solution: &'a str,
    output: &'a AnalysisOutput,
}

impl<'a> PreparedOutput<'a> {
    fn new(solution: &'a str, output: &'a AnalysisOutput) -> Self {
        Self { solution, output }
    }
}

lazy_static! {
    static ref OPTIMAL_SOLUTION_OUTPUT: AnalysisOutput =
        AnalysisOutput::new(AnalysisStatus::Approve, vec![]);
    static ref OPTIMAL_SOLUTION_WITH_EXTERN_CRATE_OUTPUT: AnalysisOutput = AnalysisOutput::new(
        AnalysisStatus::Approve,
        vec![ReverseStringComment::SuggestRemovingExternCrate.to_string()],
    );
    static ref OPTIMAL_SOLUTION_WITH_SUGGEST_BONUS_OUTPUT: AnalysisOutput = AnalysisOutput::new(
        AnalysisStatus::Approve,
        vec![ReverseStringComment::SuggestDoingBonusTest.to_string()],
    );
    static ref PREPARED_SOLUTION_OUTPUTS: Vec<PreparedOutput<'static>> = vec![
        PreparedOutput::new("use unicode_segmentation::UnicodeSegmentation; pub fn reverse(input: &str) -> String { input.graphemes(true).rev().collect() }", &OPTIMAL_SOLUTION_OUTPUT),
        PreparedOutput::new("use unicode_segmentation::UnicodeSegmentation; pub fn reverse(input: &str) -> String { input.graphemes(true).rev().collect::<String>() }", &OPTIMAL_SOLUTION_OUTPUT),
        PreparedOutput::new("pub fn reverse(input: &str) -> String { input.chars().rev().collect() }", &OPTIMAL_SOLUTION_WITH_SUGGEST_BONUS_OUTPUT),
        PreparedOutput::new("pub fn reverse(input: &str) -> String { input.chars().rev().collect::<String>() }", &OPTIMAL_SOLUTION_WITH_SUGGEST_BONUS_OUTPUT),
        PreparedOutput::new("extern crate unicode_segmentation; use unicode_segmentation::UnicodeSegmentation; pub fn reverse(input: &str) -> String { input.graphemes(true).rev().collect() }", &OPTIMAL_SOLUTION_WITH_EXTERN_CRATE_OUTPUT),
        PreparedOutput::new("extern crate unicode_segmentation; use unicode_segmentation::UnicodeSegmentation; pub fn reverse(input: &str) -> String { input.graphemes(true).rev().collect::<String>() }", &OPTIMAL_SOLUTION_WITH_EXTERN_CRATE_OUTPUT),
    ];
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
        Ok(
            if let Some(output) = PREPARED_SOLUTION_OUTPUTS
                .iter()
                .find(|prepared_output| {
                    syn::parse_str::<File>(prepared_output.solution).unwrap() == *solution_ast
                })
                .map(|prepared_output| prepared_output.output)
                .cloned()
            {
                output
            } else {
                AnalysisOutput::new(AnalysisStatus::ReferToMentor, vec![])
            },
        )
    }
}
