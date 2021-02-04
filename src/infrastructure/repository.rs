use mongodb::{Client, options::ClientOptions};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test(){

        assert!(false);
    }
}



mod manager_environment_repository {
    use crate::actor_models::environmentVariableActor::Variable;
    use async_trait::async_trait;

    #[derive(Debug)]
    struct EnvironmentRepository {
        name: String,
        status: bool,
        value: String,
    }

    #[async_trait]
    trait Actions {
        async fn insert_variable( env: EnvironmentRepository) -> Result<String, String>;
        async fn get_all_variables() -> Result<Vec<EnvironmentRepository>, String>;
        async fn get_variable(name :String) -> Result<Vec<EnvironmentRepository>, String>;
    }


    #[async_trait]
    impl Actions for EnvironmentRepository {
        async fn insert_variable(env: EnvironmentRepository) -> Result<String, String> {
            unimplemented!()
        }

        async fn get_all_variables() -> Result<Vec<EnvironmentRepository>, String> {
            unimplemented!()
        }

        async fn get_variable(name: String) -> Result<Vec<EnvironmentRepository>, String> {
            unimplemented!()
        }
    }








}