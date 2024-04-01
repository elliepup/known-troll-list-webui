use juniper::{GraphQLObject, GraphQLInputObject};

#[derive(GraphQLInputObject)]
struct CharacterDataInput {
    id: Option<i32>,
    name: Option<String>,
    server_slug: Option<String>,
    server_region: Option<String>,
}


#[derive(GraphQLObject)]
struct CharacterData {
    id: Option<i32>,
    name: Option<String>,
    server_slug: Option<String>,
    server_region: Option<String>,
}