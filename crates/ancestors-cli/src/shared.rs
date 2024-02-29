pub const DEFAULT_FRAME_RATE: f32 = 6.0;

pub type ProgressRange = std::ops::RangeInclusive<prodash::progress::key::Level>;
pub const STANDARD_RANGE: ProgressRange = 2..=2;
pub fn init_env_logger() {
    env_logger::init();
}

pub fn progress_tree(trace: bool) -> std::sync::Arc<prodash::tree::Root> {
    prodash::tree::root::Options {
        message_buffer_capacity: if trace { 10_000 } else { 200 },
        ..Default::default()
    }
    .into()
}

pub mod pretty {
    use std::io::{stderr, stdout};

    use miette::IntoDiagnostic;

    use crate::progress;
    use crate::shared::ProgressRange;

    pub fn prepare_and_run<E: std::error::Error + Send + Sync + 'static>(
        name: &str,
        trace: bool,
        verbose: bool,
        range: impl Into<Option<ProgressRange>>,
        run: impl FnOnce(
            progress::DoOrDiscard<prodash::tree::Item>,
            &mut dyn std::io::Write,
            &mut dyn std::io::Write,
        ) -> Result<(), E>,
    ) -> miette::Result<()> {
        crate::shared::init_env_logger();
        if verbose {
            let stdout = stdout();
            let mut stdout_lock = stdout.lock();
            let stderr = stderr();
            let mut stderr_lock = stderr.lock();
            run(
                progress::DoOrDiscard::from(None),
                &mut stdout_lock,
                &mut stderr_lock,
            )
            .into_diagnostic()
        } else {
            use crate::shared::{self, STANDARD_RANGE};
            let progress = shared::progress_tree(trace);
            let sub_progress = progress.add_child(name);
            // init_tracing(trace, false, &progress)?;

            let handle = shared::setup_line_renderer_range(
                &progress,
                range.into().unwrap_or(STANDARD_RANGE),
            );

            let mut out = Vec::<u8>::new();
            let mut err = Vec::<u8>::new();

            let res = 
                // gix::trace::coarse!("run").into_scope(|| {
                run(
                    progress::DoOrDiscard::from(Some(sub_progress)),
                    &mut out,
                    &mut err,
                )
            // })
            ;

            handle.shutdown_and_wait();
            std::io::Write::write_all(&mut stdout(), &out).into_diagnostic()?;
            std::io::Write::write_all(&mut stderr(), &err).into_diagnostic()?;
            res.into_diagnostic()
        }
    }
}

pub fn setup_line_renderer_range(
    progress: &std::sync::Arc<prodash::tree::Root>,
    levels: std::ops::RangeInclusive<prodash::progress::key::Level>,
) -> prodash::render::line::JoinHandle {
    prodash::render::line(
        std::io::stderr(),
        std::sync::Arc::downgrade(progress),
        prodash::render::line::Options {
            level_filter: Some(levels),
            frames_per_second: DEFAULT_FRAME_RATE,
            initial_delay: Some(std::time::Duration::from_millis(1000)),
            timestamp: true,
            throughput: true,
            hide_cursor: true,
            ..prodash::render::line::Options::default()
        }
        .auto_configure(prodash::render::line::StreamKind::Stderr),
    )
}
