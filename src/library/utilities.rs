

// Why is the error type, `E`, not constrained to some error trait, (such as `Error`)?
enum _Result<T, E/*: Error */> {
    Ok(T),
    Err(E),
}
