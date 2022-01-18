pub fn md5<S: Into<String>>(input: S) -> String {
    use crypto::digest::Digest;
    use crypto::md5::Md5;
    let mut md5 = Md5::new();
    md5.input_str(&input.into());
    md5.result_str()
}