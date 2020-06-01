mod s3;
mod schema;
mod storage;

pub fn execute(query: &str) -> Result<String, String> {
    let s = schema::create_schema();
    let jun = juniper::execute(
        query,
        None,
        &s,
        &juniper::Variables::new(),
        &schema::Context {},
    );

    let jun = match jun {
        Ok(value) => serde_json::to_string(&value).unwrap(),
        Err(error) => serde_json::to_string(&error).unwrap(),
    };
    Ok(jun)
}

#[cfg(test)]
mod tests {

    #[test]
    fn assert_me() {
        assert_eq!(String::from("A"), String::from("A"));
    }
}
