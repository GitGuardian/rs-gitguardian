use serde::{Deserialize, Serialize};

/// Status of a historical scan.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ScanStatus {
    Pending,
    Running,
    Canceled,
    Failed,
    TooLarge,
    Timeout,
    PendingTimeout,
    Finished,
}
