use juniper::{FieldResult, GraphQLEnum, GraphQLInputObject, GraphQLObject, RootNode};

#[derive(GraphQLEnum)]
enum Episode {
    NewHope,
    Empire,
    Jedi,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct Human {
    id: String,
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct NewHuman {
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
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
    fn human(context: &Context, id: String) -> FieldResult<Human> {
        // Get a db connection.
        //let connection = context.pool.get_connection()?;
        // Execute a db query.
        // Note the use of `?` to propagate errors.
        let human = Human {
            id: "123".to_owned(),
            name: "Han Solo".to_owned(),
            appears_in: vec![Episode::NewHope],
            home_planet: "Earth".to_owned(),
        };
        // Return the result.
        Ok(human)
    }
}

pub struct MutationRoot;

#[juniper::object(
    Context = Context,
)]
impl MutationRoot {
    fn createHuman(context: &Context, new_human: NewHuman) -> FieldResult<Human> {
        let human = Human {
            name: new_human.name,
            id: "123".to_owned(),
            appears_in: new_human.appears_in,
            home_planet: new_human.home_planet,
        };

        Ok(human)
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
