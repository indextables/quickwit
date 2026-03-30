// Copyright 2021-Present Datadog, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Debug utilities for conditional logging based on TANTIVY4JAVA_DEBUG environment variable

use std::env;

use chrono;
use once_cell::sync::Lazy;

/// Global debug flag for tantivy4java, evaluated once at startup
pub static TANTIVY4JAVA_DEBUG_ENABLED: Lazy<bool> = Lazy::new(|| {
    env::var("TANTIVY4JAVA_DEBUG")
        .map(|v| v == "1" || v.to_lowercase() == "true")
        .unwrap_or(false)
});

/// Format timestamp for debug output
pub fn format_timestamp() -> String {
    chrono::Utc::now().format("%H:%M:%S%.3f").to_string()
}

/// Macro for conditional debug printing when TANTIVY4JAVA_DEBUG is enabled
#[macro_export]
macro_rules! tantivy4java_debug {
    ($($arg:tt)*) => {
        if *$crate::debug::TANTIVY4JAVA_DEBUG_ENABLED {
            eprintln!("[{}] {}", $crate::debug::format_timestamp(), format!($($arg)*));
        }
    };
}
