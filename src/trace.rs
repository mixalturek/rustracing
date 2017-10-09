use std::borrow::Cow;
use std::sync::mpsc;

use span::{SpanBuilder, FinishedSpan};

#[derive(Debug)]
pub struct Tracer<T> {
    span_tx: mpsc::Sender<FinishedSpan<T>>,
}
impl<T> Tracer<T> {
    pub fn span(&self, operation_name: Cow<'static, str>, state: T) -> SpanBuilder<T> {
        SpanBuilder::new(operation_name, state, self.span_tx.clone())
    }
}
impl<T> Clone for Tracer<T> {
    fn clone(&self) -> Self {
        Tracer { span_tx: self.span_tx.clone() }
    }
}
