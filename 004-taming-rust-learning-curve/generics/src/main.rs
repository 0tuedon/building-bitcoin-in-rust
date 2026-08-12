use std::marker::PhantomData;

use ::hex;
use ::serde::Serialize;
struct Json;
struct Toml;
struct Cbor;
struct Yaml;

#[derive(Serialize)]
struct User<T: Encode> {
    name: String,
    age: u32,
    #[serde(skip)]
    _marker: PhantomData<T>,
}

trait Encode {
    fn encode<T: Serialize>(val: T) -> String;
}

impl Encode for Json {
    fn encode<T: Serialize>(val: T) -> String {
        serde_json::to_string(&val).unwrap()
    }
}

impl Encode for Toml {
    fn encode<T: Serialize>(val: T) -> String {
        toml::to_string(&val).unwrap()
    }
}

impl Encode for Cbor {
    fn encode<T: Serialize>(val: T) -> String {
        let bytes = serde_cbor::to_vec(&val).unwrap();
        hex::encode(bytes)
    }
}

impl Encode for Yaml {
    fn encode<T: Serialize>(val: T) -> String {
        serde_yaml::to_string(&val).unwrap()
    }
}

impl<T> User<T>
where
    T: Encode,
{
    fn new(name: String, age: u32) -> Self {
        User {
            name,
            age,
            _marker: PhantomData,
        }
    }
}

fn main() {
    let user = User::<Json>::new("Alice".to_string(), 30);
    let encoded = Json::encode(&user);
    println!("{:?}", encoded);
}
