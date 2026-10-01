use super::*;

#[test]
fn errors_score_highest() {
    assert_eq!(line_score("FATAL: connection refused"), SCORE_ERROR);
    assert_eq!(line_score("thread panicked at 'boom'"), SCORE_ERROR);
    assert!(line_score("error: mismatched types") >= SCORE_ERROR);
}

#[test]
fn warnings_below_errors_above_baseline() {
    let w = line_score("warning: unused variable");
    assert!(w < SCORE_ERROR);
    assert!(w > SCORE_BASELINE);
}

#[test]
fn plain_line_is_baseline() {
    assert_eq!(line_score("   Compiling foo v0.1.0"), SCORE_BASELINE);
}

#[test]
fn severity_buckets() {
    assert_eq!(severity("error[E0382]: borrow"), Severity::Error);
    assert_eq!(severity("warning: deprecated"), Severity::Warning);
    assert_eq!(severity("running 12 tests"), Severity::Other);
}

#[test]
fn explicit_level_wins_over_keyword_substring() {
    // `WARN` level pins the line as a warning even though it mentions
    // `exception` — the old substring scan wrongly bucketed this as Error.
    assert_eq!(
        severity(
            "081109 214043 2561 WARN dfs.DataNode: Got exception while serving blk_1 to /10.0.0.1:"
        ),
        Severity::Warning
    );
    assert_eq!(
        severity("[WARN] task failed, will retry"),
        Severity::Warning
    );
    assert_eq!(
        severity("INFO: request failed but retried ok"),
        Severity::Other
    );
    assert_eq!(
        severity("level=warn msg=\"exception caught\""),
        Severity::Warning
    );
}

#[test]
fn explicit_error_level_still_error() {
    assert_eq!(severity("ERROR something broke"), Severity::Error);
    assert_eq!(severity("2024-01-01 FATAL boom"), Severity::Error);
    assert_eq!(severity("level=error msg=oops"), Severity::Error);
}

#[test]
fn keyword_needs_word_boundary() {
    // `error_count` / `errors_total` are identifiers, not an error signal.
    assert_eq!(severity("metrics error_count=0 ok"), Severity::Other);
    assert_eq!(severity("stat errors_total 0"), Severity::Other);
    // A real error keyword at a boundary still classifies.
    assert_eq!(severity("fatal: repository not found"), Severity::Error);
    assert_eq!(
        severity("connection (error) while dialing"),
        Severity::Error
    );
}

#[test]
fn bare_fail_is_at_least_warning() {
    // HealthApp/logcat style line: no level token, lowercase `fail`.
    assert_eq!(
        severity(
            "20171223-22:19:58:380|HiH_HiHealthDataInsertStore|30002312|saveHealthDetailData() saveOneDetailData fail hiHealthData = 1513958400000,type = 40003"
        ),
        Severity::Warning
    );
    // Word boundary: `failover` is not a failure signal.
    assert_eq!(severity("switched to failover replica"), Severity::Other);
    // An explicit level still wins over the keyword.
    assert_eq!(severity("INFO retry ok after fail"), Severity::Other);
}

#[test]
fn has_error_indicators_detects() {
    assert!(has_error_indicators("test result: FAILED"));
    assert!(!has_error_indicators("all good, 12 passed"));
}
