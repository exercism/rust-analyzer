use rust_analyzer::analyzers;
use std::fmt::Display;

const WEBSITE_COPY_URL_PREFIX: &str =
    "https://api.github.com/repos/exercism/website-copy/contents/automated-comments";

fn check_if_comments_exist(comments: &[impl Display]) {
    comments
        .iter()
        .map(|comment| {
            format!(
                "{}/{}.md",
                WEBSITE_COPY_URL_PREFIX,
                comment.to_string().replace(".", "/")
            )
        })
        .for_each(|url| {
            assert!(
                reqwest::get(&url).unwrap().status().is_success(),
                "failed to locate the comment at the website-copy repository; url = {}",
                url
            )
        });
}

#[test]
fn general_analyzer_comments_are_present_at_website_copy() {
    use analyzers::comments::GeneralComment::*;
    check_if_comments_exist(&[SolutionFileNotFound, FailedToParseSolutionFile]);
}

#[test]
fn reverse_string_analyzer_comments_are_present_at_website_copy() {
    use analyzers::reverse_string::comments::ReverseStringComment::*;
    check_if_comments_exist(&[
        SuggestRemovingExternCrate,
        SuggestDoingBonusTest,
        SolutionFunctionNotFound,
    ]);
}
