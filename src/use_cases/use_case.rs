pub trait UseCase<TPayload, TResult> {
    fn execute(&self, payload: TPayload) -> TResult;
}
