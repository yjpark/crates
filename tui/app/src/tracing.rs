use tracing_log::AsTrace;
use edger_args::prelude::VerboseArg;

pub fn init(verbose: &VerboseArg) {
    tracing_subscriber::fmt()
        .with_max_level(verbose.log_level_filter().as_trace())
        .with_file(true)
        .with_line_number(true)
        .with_target(true)
        .with_thread_ids(true)
        .init();
}
