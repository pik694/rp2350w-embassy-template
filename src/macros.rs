#[macro_export]
macro_rules! measure_time {
    {$desc:literal, $block:block} => {{
        use $crate::logging::debug;

        debug!("{}: begin", $desc);
        let start = embassy_time::Instant::now();
        let r = $block;
        let end = embassy_time::Instant::now();
        let duration = end - start;
        debug!("{}: done in {} ms", $desc, duration.as_millis());

        r
    }};
}
