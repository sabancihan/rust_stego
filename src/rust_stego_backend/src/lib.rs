mod core;

#[ic_cdk::query]
fn encrypt(input: String, base64_image: String) -> String {
    core::hide_secret(input, base64_image)
}

#[ic_cdk::query]
fn decrypt(base64_image: String) -> String {
    core::find_secret(base64_image)
}


