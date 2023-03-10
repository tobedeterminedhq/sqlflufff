// use super::pos::PosMarker;

type CheckTuple = (String, usize, usize);

struct SQLBaseError {
    fatal: bool,
    ignore: bool,
    warning: bool,
    line_no: usize,
    line_pos: usize,
}

// impl SQLBaseError {
// /// Should this error be considered fixable?
// fn fixable(&self) -> bool {
//     false
// }

// /// Fetch the code of the rule which cause this error.
// /// NB: This only returns a real code for some subclasses of
// /// error, (the ones with a `rule` attribute), but otherwise
// /// returns a placeholder value which can be used instead.
// fn rule_code(&self) -> String {
//     // TODO
//     "".to_string()
// }
// }
//     /// Fetch a description of this violation.
//     /// NB: For violations which don't directly implement a rule
//     /// this attempts to return the error message linked to whatever
//     /// caused the violation. Optionally some errors may have their
//     /// description set directly.
//     fn desc(&self) -> String {
//         // TODO
//         "".to_string()
//     }
//
//     /// Return a dict of properties.
//     /// This is useful in the API for outputting violations.
//     fn get_info_dict(&self) -> () {
//         // TODO
//         ()
//     }
//
//     /// Get a tuple representing this error. Mostly for testing.
//     fn check_tuple(&self) -> CheckTuple {
//         // TODO
//         ("".to_string(), 0, 0)
//     }
//
//     /// Return hashable source signature for deduplication.
//     fn source_signature(&self) -> () {
//         // TODO
//         ()
//     }
//
//     /// Ignore this violation if it matches the iterable.
//     fn ignore_if_in(&self, ignore_iterable: Vec<String>) {
//         // TODO
//     }
//
//     /// Warning only for this violation if it matches the iterable.
//     /// Designed for rule codes so works with L001, L00X but also TMP or PRS
//     /// for templating and parsing errors.
//     fn warning_if_in(&self, warning_iterable: Vec<String>) {
//         // TODO
//     }
// }
//
// struct SQLTemplaterError {
//     pos: Option<PosMarker>,
// }
//
// impl SQLTemplaterError {
//     /// An error which occurred during templating.
//     /// Args:
//     ///     pos (:obj:`PosMarker`, optional): The position which the error
//     ///         occurred at.
//     fn new(pos: Option<PosMarker>) -> SQLTemplaterError {
//         SQLTemplaterError { pos }
//     }
// }
//
// #[derive(Debug)]
// struct SQLFluffSkipFile;
//
// impl Error for SQLFluffSkipFile {}
//
// impl fmt::Display for SQLFluffSkipFile {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "An error returned from a templater to skip a file.")
//     }
// }
//
// #[derive(Debug)]
// struct SQLLexError {
//     pos: PosMarker,
// }
//
// impl Error for SQLLexError {}
//
// impl fmt::Display for SQLLexError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "An error which occurred during lexing.")
//     }
// }
//
// #[derive(Debug)]
// struct SQLParseError {
//     segment: BaseSegment,
// }
//
// impl Error for SQLParseError {}
//
// impl fmt::Display for SQLParseError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "An error which occurred during parsing.")
//     }
// }
//
// #[derive(Debug)]
// struct SQLLintError {
//     segment: BaseSegment,
//     rule: Rule,
//     fixes: Vec<Fix>,
//     description: String,
// }
//
// impl Error for SQLLintError {}
//
// impl fmt::Display for SQLLintError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "An error which occurred during linting.")
//     }
// }
//
// #[derive(Debug)]
// struct SQLFluffUserError;
//
// impl Error for SQLFluffUserError {}
//
// impl fmt::Display for SQLFluffUserError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "An error which should be fed back to the user.")
//     }
// }

/// An error which should be fed back to the user.
struct SQLFluffUserError {
    value: String,
}

// Not from SQLFluff but translates Pythn value error
#[derive(Debug)]
pub struct ValueError {
    value: String,
}

impl ValueError {
    pub fn new(value: String) -> ValueError {
        ValueError { value }
    }
}
