use std::cmp::Ordering;

// TODO: чтобы включить работу док-тестов, нужно вынести в крейт с таргетом lib. В кору нести, видимо.
/// Produces multiple strings from a given list of possible values which are similar
/// to the passed in value `v` within a certain confidence by least confidence.
///
/// Example when suggest may be positive:
/// ```
/// const GOOD_VARIANTS: [&str; 2] = ["--config", "--help"];
/// let bad_input = "comfig"; // let's imagine that the input comes from user
///
/// let suggest = calculate_suggestions(bad_input, GOOD_VARIANTS);
/// assert_eq!(Some(&"--config".to_string()), suggest.get(0))
/// ```
///
/// Example when suggest may be negative
/// ```
/// const GOOD_VARIANTS: [&str; 2] = ["--config", "--help"];
/// let bad_input = "something"; // let's imagine that the input comes from user
///
/// let suggest = calculate_suggestions(bad_input, GOOD_VARIANTS);
/// assert_eq!(None, suggest.get(0))
/// ```
pub(crate) fn calculate_suggestions<T, I>(v: &str, possible_values: I) -> Vec<String>
where
    T: AsRef<str>,
    I: IntoIterator<Item = T>,
{
    let mut candidates: Vec<(f64, String)> = possible_values
        .into_iter()
        .map(|pv| (strsim::jaro_winkler(v, pv.as_ref()), pv.as_ref().to_owned()))
        .filter(|(confidence, _)| confidence > &0.8)
        .collect();
    candidates.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal));
    candidates.into_iter().map(|(_, pv)| pv).collect()
}

#[allow(unused_imports)]
// #[cfg(tests)]
mod tests {
    use crate::utility::suggestions::calculate_suggestions;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_suggestion_hit() {
        const GOOD_VARIANTS: [&str; 2] = ["--config", "--help"];
        let bad_input = "comfig"; // let's imagine that the input comes from user

        let suggest = calculate_suggestions(bad_input, GOOD_VARIANTS);
        assert_eq!(Some(&"--config".to_string()), suggest.get(0))
    }

    #[test]
    fn test_suggestion_miss() {
        const GOOD_VARIANTS: [&str; 2] = ["--config", "--help"];
        let bad_input = "something"; // let's imagine that the input comes from user

        let suggest = calculate_suggestions(bad_input, GOOD_VARIANTS);
        assert_eq!(None, suggest.get(0))
    }
}
