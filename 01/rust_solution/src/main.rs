use base64::{decode, encode};
use rand::distributions::Alphanumeric;
use rand::Rng;

fn main() {
    // Generate a random string
    let random_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();

    println!("Random String: {}", random_string);

    // Encode the string
    let encoded_string = encode(&random_string);
    println!("Encoded String: {}", encoded_string);

    // Decode the string
    match decode(&encoded_string) {
        Ok(decoded_bytes) => {
            //We get the result in bytes, so we convert the decoded bytes to a String
            let decoded_string = String::from_utf8(decoded_bytes).unwrap();
            println!("Decoded String: {}", decoded_string);
        }
        Err(e) => println!("Failed to decode: {:?}", e),
    }
}
