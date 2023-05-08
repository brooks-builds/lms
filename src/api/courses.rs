use graphql_client::GraphQLQuery;
use ycl::foundations::roles::BBRole;
use yew::AttrValue;

use crate::{
    database_queries::{api_get_all_data, ApiGetAllData},
    types::{ApiAllData, Course},
};

use super::SendToGraphql;
