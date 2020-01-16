use rust_lambda_graphql;
#[test]
fn it_works(){

    let re = rust_lambda_graphql::graphql::execute().unwrap();
    assert_eq!(String::from("Good"),re);
}