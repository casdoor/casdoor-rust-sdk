// Copyright 2026 The Casdoor Authors. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::time::{SystemTime, UNIX_EPOCH};

/// Get the current time in RFC 3339 format (UTC), e.g. `2026-01-02T15:04:05Z`, which is the
/// format Casdoor uses for fields like `created_time`.
pub fn get_current_time() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs() as i64)
        .unwrap_or_default();

    format_rfc3339(secs)
}

fn format_rfc3339(secs: i64) -> String {
    let days = secs.div_euclid(86400);
    let secs_of_day = secs.rem_euclid(86400);
    let (year, month, day) = civil_from_days(days);

    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}Z",
        secs_of_day / 3600,
        secs_of_day % 3600 / 60,
        secs_of_day % 60
    )
}

// Converts days since 1970-01-01 to a (year, month, day) date, see
// https://howardhinnant.github.io/date_algorithms.html#civil_from_days
fn civil_from_days(days: i64) -> (i64, i64, i64) {
    let z = days + 719468;
    let era = z.div_euclid(146097);
    let doe = z.rem_euclid(146097);
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);

    (year, month, day)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_rfc3339() {
        assert_eq!(format_rfc3339(0), "1970-01-01T00:00:00Z");
        assert_eq!(format_rfc3339(951782400), "2000-02-29T00:00:00Z");
        assert_eq!(format_rfc3339(1767366245), "2026-01-02T15:04:05Z");
    }
}
