use std::env;
use std::io::{IsTerminal, Write};

use log::{Level, LevelFilter, Log, Metadata, Record};

const WORKSPACE_CRATES: &[&str] = &["qrust_run", "run", "parse", "ast", "typed", "clif"];

#[derive(Debug, Clone, PartialEq, Eq)]
struct LogFilter {
    workspace_default: LevelFilter,
    external_default: LevelFilter,
    rules: Vec<(String, LevelFilter)>,
}

impl Default for LogFilter {
    fn default() -> Self {
        Self {
            workspace_default: LevelFilter::Info,
            external_default: LevelFilter::Warn,
            rules: Vec::new(),
        }
    }
}

impl LogFilter {
    fn parse(spec: &str) -> (Self, Vec<String>) {
        let mut filter = Self::default();
        let mut warnings = Vec::new();

        for raw in spec.split(',') {
            let directive = raw.trim();
            if directive.is_empty() {
                continue;
            }

            if let Some((target, level_str)) = directive.split_once('=') {
                let target = target.trim();
                let Some(level) = parse_level_filter(level_str.trim()) else {
                    warnings.push(format!("ignoring invalid log level in `{directive}`"));
                    continue;
                };
                match target {
                    "*" | "all" => {
                        filter.workspace_default = level;
                        filter.external_default = level;
                    }
                    "qrust" => {
                        filter.workspace_default = level;
                    }
                    _ => {
                        let normalized = if target == "run" { "qrust_run" } else { target };
                        if let Some(existing) = filter.rules.iter_mut().find(|(t, _)| t == normalized) {
                            existing.1 = level;
                        } else {
                            filter.rules.push((normalized.to_string(), level));
                        }
                    }
                }
            } else if let Some(level) = parse_level_filter(directive) {
                filter.workspace_default = level;
                if level == LevelFilter::Off {
                    filter.external_default = LevelFilter::Off;
                }
            } else {
                warnings.push(format!("ignoring invalid log directive `{directive}`"));
            }
        }

        (filter, warnings)
    }

    fn max_level(&self) -> LevelFilter {
        let mut max = self.workspace_default.max(self.external_default);
        for (_, level) in &self.rules {
            max = max.max(*level);
        }
        max
    }

    fn level_for(&self, target: &str) -> LevelFilter {
        let mut best: Option<(usize, LevelFilter)> = None;
        for (prefix, level) in &self.rules {
            if matches_target(target, prefix)
                && best.is_none_or(|(best_len, _)| prefix.len() > best_len)
            {
                best = Some((prefix.len(), *level));
            }
        }
        if let Some((_, level)) = best {
            return level;
        }
        if is_workspace_target(target) {
            self.workspace_default
        } else {
            self.external_default
        }
    }
}

fn matches_target(target: &str, prefix: &str) -> bool {
    target.starts_with(prefix)
        && (target.len() == prefix.len() || target[prefix.len()..].starts_with("::"))
}

fn is_workspace_target(target: &str) -> bool {
    WORKSPACE_CRATES
        .iter()
        .any(|crate_name| matches_target(target, crate_name))
}

fn parse_level_filter(s: &str) -> Option<LevelFilter> {
    match s.to_ascii_lowercase().as_str() {
        "off" => Some(LevelFilter::Off),
        "error" => Some(LevelFilter::Error),
        "warn" => Some(LevelFilter::Warn),
        "info" => Some(LevelFilter::Info),
        "debug" => Some(LevelFilter::Debug),
        "trace" => Some(LevelFilter::Trace),
        _ => None,
    }
}

struct QrustLogger {
    filter: LogFilter,
    color: bool,
}

impl Log for QrustLogger {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        metadata.level() <= self.filter.level_for(metadata.target())
    }

    fn log(&self, record: &Record<'_>) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let mut stderr = std::io::stderr().lock();
        if self.color {
            let (level_style, reset, dim) = (level_color(record.level()), "\x1b[0m", "\x1b[2m");
            let _ = writeln!(
                stderr,
                "{dim}[{reset}{level_style}{:<5}{reset} {dim}{}]{reset} {}",
                record.level(),
                record.target(),
                record.args()
            );
        } else {
            let _ = writeln!(
                stderr,
                "[{:<5} {}] {}",
                record.level(),
                record.target(),
                record.args()
            );
        }
    }

    fn flush(&self) {
        let _ = std::io::stderr().flush();
    }
}

fn level_color(level: Level) -> &'static str {
    match level {
        Level::Error => "\x1b[1;31m",
        Level::Warn => "\x1b[33m",
        Level::Info => "\x1b[32m",
        Level::Debug => "\x1b[36m",
        Level::Trace => "\x1b[35m",
    }
}

fn should_use_color() -> bool {
    match env::var("QRUST_LOG_STYLE").ok().as_deref() {
        Some("always") => return true,
        Some("never") => return false,
        _ => {}
    }
    if env::var_os("NO_COLOR").is_some_and(|v| !v.is_empty()) {
        return false;
    }
    std::io::stderr().is_terminal()
}

/// Initializes global logging from `QRUST_LOG` (falling back to `RUST_LOG`).
/// Defaults to `info` for workspace crates and `warn` for third-party dependencies.
pub fn init() {
    let spec = env::var("QRUST_LOG")
        .or_else(|_| env::var("RUST_LOG"))
        .unwrap_or_default();

    let (filter, warnings) = if spec.trim().is_empty() {
        (LogFilter::default(), Vec::new())
    } else {
        LogFilter::parse(&spec)
    };

    let max_level = filter.max_level();
    let logger: &'static QrustLogger = Box::leak(Box::new(QrustLogger {
        filter,
        color: should_use_color(),
    }));
    
    if log::set_logger(logger).is_ok() {
        log::set_max_level(max_level);
        for warning in warnings {
            log::warn!("{warning}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_filter_keeps_third_party_at_warn() {
        let filter = LogFilter::default();
        assert_eq!(filter.level_for(module_path!()), LevelFilter::Info);
        assert_eq!(filter.level_for("qrust_run"), LevelFilter::Info);
        assert_eq!(filter.level_for("ast::ast_visitor"), LevelFilter::Info);
        assert_eq!(filter.level_for("cranelift_codegen::ir"), LevelFilter::Warn);
    }

    #[test]
    fn bare_level_only_raises_workspace_crates() {
        let (filter, warnings) = LogFilter::parse("trace");
        assert!(warnings.is_empty());
        assert_eq!(filter.level_for("typed::typecheck"), LevelFilter::Trace);
        assert_eq!(filter.level_for("cranelift_codegen"), LevelFilter::Warn);
    }

    #[test]
    fn targeted_longest_prefix_wins() {
        let (filter, warnings) = LogFilter::parse("clif=off,clif::codegen::matrix=debug,cranelift_codegen=info");
        assert!(warnings.is_empty());
        assert_eq!(filter.level_for("clif::codegen"), LevelFilter::Off);
        assert_eq!(filter.level_for("clif::codegen::matrix"), LevelFilter::Debug);
        assert_eq!(filter.level_for("cranelift_codegen::isa"), LevelFilter::Info);
    }

    #[test]
    fn wildcard_and_qrust_alias() {
        let (all_trace, _) = LogFilter::parse("*=trace");
        assert_eq!(all_trace.level_for("cranelift_codegen"), LevelFilter::Trace);
        assert_eq!(all_trace.level_for("ast"), LevelFilter::Trace);

        let (qrust_debug, _) = LogFilter::parse("qrust=debug");
        assert_eq!(qrust_debug.level_for("clif::codegen"), LevelFilter::Debug);
        assert_eq!(qrust_debug.level_for("regalloc2"), LevelFilter::Warn);
    }

    #[test]
    fn off_silences_everything() {
        let (filter, _) = LogFilter::parse("off");
        assert_eq!(filter.max_level(), LevelFilter::Off);
        assert_eq!(filter.level_for("run"), LevelFilter::Off);
        assert_eq!(filter.level_for("cranelift_codegen"), LevelFilter::Off);
    }
}
