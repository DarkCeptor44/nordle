// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

const ANSWERS: &str = include_str!("../../assets/answers.txt");
const ALLOWED: &str = include_str!("../../assets/allowed.txt");

/// Check if a word is valid
///
/// ## Arguments
///
/// * `word` - The word to check
///
/// ## Returns
///
/// * `bool` - `true` if the word is valid, `false` otherwise
#[must_use]
pub fn is_valid_word<S>(word: S) -> bool
where
    S: AsRef<str>,
{
    let w = word.as_ref().trim();
    ANSWERS.contains(w) || ALLOWED.contains(w)
}

/// Get a list of possible answers
///
/// ## Returns
///
/// * `Vec<&'static str>` - A list of possible answers
#[must_use]
pub fn words() -> Vec<&'static str> {
    ANSWERS.lines().collect()
}
