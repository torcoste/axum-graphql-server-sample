use async_graphql::Object;

static mut USER_NAME: &str = "John Doe";

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn get_name(&self) -> &str {
        unsafe { USER_NAME }
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn set_name(&self, name: String) -> &str {
        unsafe {
            USER_NAME = Box::leak(name.clone().into_boxed_str());
        }
        unsafe { USER_NAME }
    }
}
