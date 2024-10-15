cfg_if! {
    if #[cfg(feature = "uart_hyp")] {
    pub mod uarthyp;
    }
}
