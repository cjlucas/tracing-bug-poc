fn main() {
    tracing_subscriber::fmt::init();

    println!("=== good ===");
    good();

    println!();
    println!("=== bad ===");
    bad();
}

fn good() {
    let span1 = tracing::error_span!("span1");
    let _enter1 = span1.enter();

    let span2 = tracing::error_span!("span2");
    let _enter2 = span2.enter();

    drop(_enter2);
    drop(span2);

    drop(_enter1);
    drop(span1);
}

fn bad() {
    let span1 = tracing::error_span!("span1");
    let _enter1 = span1.enter();

    let span2 = tracing::error_span!("span2");
    let _enter2 = span2.enter();

    drop(_enter1);
    drop(span1);

    drop(_enter2);
    drop(span2);
}
