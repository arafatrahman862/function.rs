pub fn execute_fut<F: std::future::Future>(body: F) -> F::Output {
    return tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Failed building the Runtime")
        .block_on(body);
}
