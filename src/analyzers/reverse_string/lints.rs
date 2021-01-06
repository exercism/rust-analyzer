//! #lints
//! This module contains lints that are used on a "reverse-string" solution.
//! Each lint performs a single check on an input solution AST and, depending on the check result,
//! modifies the output analysis.
//! Each lint assumes that the solution contains a properly declared "reverse" function.

use crate::{
    analyzers::{
        output::{AnalysisOutput, AnalysisStatus},
        reverse_string::comments::ReverseStringComment,
    },
    Result,
};

use syn::File;

pub static REVERSE_STRING_LINTS: &[fn(&File, &mut AnalysisOutput) -> Result<()>] = &[
    check_for_extern_crate,
    check_for_standard_solution,
    check_for_optimal_solution,
];

fn check_for_extern_crate(ast: &File, output: &mut AnalysisOutput) -> Result<()> {
    let has_extern = ast
        .items
        .iter()
        .any(|item| matches!(item, syn::Item::ExternCrate(_)));
    if has_extern {
        output
            .comments
            .push(ReverseStringComment::SuggestRemovingExternCrate.to_string());
    }
    Ok(())
}

fn check_for_standard_solution(ast: &File, output: &mut AnalysisOutput) -> Result<()> {
    let reverse_fn = if let Some(syn::Item::Fn(func)) = ast.items.iter().find(|item| {
        if let syn::Item::Fn(item_fn) = item {
            item_fn.ident == "reverse"
        } else {
            false
        }
    }) {
        func
    } else {
        // We can use the unreachable!, since we have already check for
        // the "reverse" function existence at the analyze function.
        unreachable!();
    };
    // Check if the "reverse" function contains the "input.chars().rev().collect()"
    // solution
    let has_standard_solution = reverse_fn.block.stmts.iter().any(|stmt| {
        if let syn::Stmt::Expr(syn::Expr::MethodCall(syn::ExprMethodCall {
            method,
            receiver,
            ..
        })) = stmt
        {
            if method != "collect" {
                false
            } else if let syn::Expr::MethodCall(syn::ExprMethodCall {
                method, receiver, ..
            }) = receiver.as_ref()
            {
                if method != "rev" {
                    false
                } else if let syn::Expr::MethodCall(syn::ExprMethodCall {
                    method, receiver, ..
                }) = receiver.as_ref()
                {
                    if method != "chars" {
                        false
                    } else if let syn::Expr::Path(syn::ExprPath { path, .. }) = receiver.as_ref() {
                        if let Some(syn::punctuated::Pair::End(path_segment)) =
                            path.segments.first()
                        {
                            if let Some(syn::punctuated::Pair::End(syn::FnArg::Captured(
                                syn::ArgCaptured {
                                    pat: syn::Pat::Ident(syn::PatIdent { ident, .. }),
                                    ..
                                },
                            ))) = reverse_fn.decl.inputs.first()
                            {
                                *ident == path_segment.ident
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
                }
            } else {
                false
            }
        } else {
            false
        }
    });
    if has_standard_solution {
        output.status = AnalysisStatus::Approve;
        output
            .comments
            .push(ReverseStringComment::SuggestDoingBonusTest.to_string())
    }
    Ok(())
}

fn check_for_optimal_solution(ast: &File, output: &mut AnalysisOutput) -> Result<()> {
    let reverse_fn = if let Some(syn::Item::Fn(func)) = ast.items.iter().find(|item| {
        if let syn::Item::Fn(item_fn) = item {
            item_fn.ident == "reverse"
        } else {
            false
        }
    }) {
        func
    } else {
        // We can use the unreachable!, since we have already check for
        // the "reverse" function existence at the analyze function.
        unreachable!();
    };
    // Check if the "reverse" function contains the "input.graphemes(true).rev().collect()"
    // solution
    let has_optimal_solution = reverse_fn.block.stmts.iter().any(|stmt| {
        if let syn::Stmt::Expr(syn::Expr::MethodCall(syn::ExprMethodCall {
            method,
            receiver,
            ..
        })) = stmt
        {
            if method != "collect" {
                false
            } else if let syn::Expr::MethodCall(syn::ExprMethodCall {
                method, receiver, ..
            }) = receiver.as_ref()
            {
                if method != "rev" {
                    false
                } else if let syn::Expr::MethodCall(syn::ExprMethodCall {
                    method,
                    receiver,
                    args,
                    ..
                }) = receiver.as_ref()
                {
                    if method != "graphemes" {
                        false
                    } else if let Some(syn::punctuated::Pair::End(syn::Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Bool(syn::LitBool { value, .. }),
                        ..
                    }))) = args.first()
                    {
                        *value
                    } else if let syn::Expr::Path(syn::ExprPath { path, .. }) = receiver.as_ref() {
                        if let Some(syn::punctuated::Pair::End(path_segment)) =
                            path.segments.first()
                        {
                            if let Some(syn::punctuated::Pair::End(syn::FnArg::Captured(
                                syn::ArgCaptured {
                                    pat: syn::Pat::Ident(syn::PatIdent { ident, .. }),
                                    ..
                                },
                            ))) = reverse_fn.decl.inputs.first()
                            {
                                *ident == path_segment.ident
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
                }
            } else {
                false
            }
        } else {
            false
        }
    });
    let has_use_item = if let Some(syn::Item::Use(syn::ItemUse {
        tree: syn::UseTree::Path(syn::UsePath { ident, tree, .. }),
        ..
    })) = ast
        .items
        .iter()
        .find(|item| matches!(item, syn::Item::Use(_)))
    {
        ident == "unicode_segmentation"
            && if let syn::UseTree::Name(syn::UseName { ident }) = tree.as_ref() {
                ident == "UnicodeSegmentation"
            } else {
                false
            }
    } else {
        false
    };
    if has_optimal_solution && has_use_item {
        output.status = AnalysisStatus::Approve;
    }
    Ok(())
}
