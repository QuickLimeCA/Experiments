extern crate serde;

extern crate serde_json;

#[macro_use]
extern crate serde_derive;


pub fn main() {
    let v1_data = r#"{"test_prop": "42"}"#;
    let v2_data = r#"{"test_prop": 42}"#;
    let v3_data = r#"{"test_prop": [42]}"#;

    let v1: Result<MyStruct, serde_json::Error> = serde_json::from_str(v1_data);
    let v2: Result<MyStruct, serde_json::Error> = serde_json::from_str(v2_data);
    let v3: Result<MyStruct, serde_json::Error> = serde_json::from_str(v3_data);

    #[allow(unreachable_patterns)]
    match v1 {
        Ok(MyStruct::V1(data)) => println!("V1: {:?}", data.test_prop),
        Ok(MyStruct::V2(data)) => println!("V2: {:?}", data.test_prop),
        Ok(MyStruct::V3(data)) => println!("V3: {:?}", data.test_prop),
        Err(err) => println!("Couldn't parse: {:?}", err),
        _ => println!("Unknown version")
    }

    #[allow(unreachable_patterns)]
    match v2 {
        Ok(MyStruct::V1(data)) => println!("V1: {:?}", data.test_prop),
        Ok(MyStruct::V2(data)) => println!("V2: {:?}", data.test_prop),
        Ok(MyStruct::V3(data)) => println!("V3: {:?}", data.test_prop),
        Err(err) => println!("Couldn't parse: {:?}", err),
        _ => println!("Unknown version")
    }

    #[allow(unreachable_patterns)]
    match v3 {
        Ok(MyStruct::V1(data)) => println!("V1: {:?}", data.test_prop),
        Ok(MyStruct::V2(data)) => println!("V2: {:?}", data.test_prop),
        //Ok(MyStruct::V3(data)) => println!("V3: {:?}", data.test_prop),
        Err(err) => println!("Couldn't parse: {:?}", err),
        _ => println!("Unknown version")
    }
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MyStruct1 {
    pub test_prop: String
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MyStruct2 {
    pub test_prop: i32
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MyStruct3 {
    pub test_prop: Vec<i64>
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum MyStruct {
    V1(MyStruct1),
    V2(MyStruct2),
    V3(MyStruct3)
}