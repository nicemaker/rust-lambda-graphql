mod schema;


pub fn execute(query: &str) -> Result<String, String> {

    juniper::execute(
        query,
        None,
        &schema::create_schema(),
        &juniper::Variables::new(),
        &schema::Context{},
    );
    Ok(String::from("Good"))
}
