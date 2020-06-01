use juniper::{FieldResult, GraphQLEnum, GraphQLInputObject, GraphQLObject, RootNode};
use serde::{Deserialize, Serialize};

#[derive(GraphQLEnum)]
enum Episode {
    NewHope,
    Empire,
    Jedi,
}

#[derive(GraphQLObject, Serialize, Deserialize, Debug, PartialEq)]
#[graphql(description = "Some Skill Gathered During your Lifetime")]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub group: String,
    pub level: i32,
    pub description: Option<String>,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Some Skill Gathered During your Lifetime")]
struct SkillPut {
    id: Option<String>,
    name: String,
    group: String,
    level: i32,
    description: Option<String>,
}

pub struct Context {}
// To make our context usable by Juniper, we have to implement a marker trait.
impl juniper::Context for Context {}

pub struct QueryRoot;

#[juniper::object(
    // Here we specify the context type for this object.
    Context = Context,
)]
impl QueryRoot {
    fn apiVersion() -> &str {
        "1.0"
    }

    // Arguments to resolvers can either be simple types or input objects.
    // To gain access to the context, we specify a argument
    // that is a reference to the Context type.
    // Juniper automatically injects the correct context here.
    fn skill(context: &Context, id: String) -> FieldResult<Skill> {
        // Get a db connection.
        //let connection = context.pool.get_connection()?;
        // Execute a db query.
        // Note the use of `?` to propagate errors.
        let skill = Skill {
            id: "123".to_owned(),
            name: "Han Solo".to_owned(),
            group: "AWS".to_owned(),
            level: 1,
            description: None,
        };
        // Return the result.
        Ok(skill)
    }
}

pub struct MutationRoot;

#[juniper::object(
    Context = Context,
)]
impl MutationRoot {
    fn skillPut(context: &Context, skill: SkillPut) -> FieldResult<Skill> {
        Ok(Skill {
            id: String::from("123"),
            name: String::from("bla"),
            group: String::from("bla"),
            level: 1,
            description: None,
        })
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
