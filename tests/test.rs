use rust_lambda_graphql as rlq;
#[test]
fn it_works() {
    let query = r#"query{skill(id:"123"){name}}"#;
    let re = rlq::execute(query).unwrap();
    assert_eq!(String::from("[{\"skill\":{\"name\":\"Han Solo\"}},[]]"), re);
}
