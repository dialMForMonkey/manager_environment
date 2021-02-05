
#[cfg(test)]
mod test_integration {
    use crate::infrastructure::repository::manager_environment_repository;
    use crate::infrastructure::repository::manager_environment_repository::ActionsDB;
    use tokio;
    use std::env;
    use dotenv::dotenv;

    #[tokio::test]
    async fn insert_variable_test(){
        let a  = dotenv::from_filename("test_integration.env").unwrap();

       let env:  manager_environment_repository::EnvironmentRepository =   manager_environment_repository::EnvironmentRepository {
            name: "teste".to_string(),
            status:false,
            value: "teste".to_string()
       };
        let actions = manager_environment_repository::ActionsDB::new(env);

        let result_insert = actions.insert_variable().await;

        assert!(result_insert.is_ok())

    }
}



pub mod manager_environment_repository {
    use crate::actor_models::environmentVariableActor::Variable;
    use async_trait::async_trait;
    use mongodb::{Client, options::ClientOptions};
    use mongodb::bson::doc;


    async fn get_client_mongo() -> Client {

        let conection_string = std::env::var("MONGO_URL").unwrap();
        let result_options = ClientOptions::parse(conection_string.as_str()).await;

        match result_options {
            Ok(options)=>{
                match Client::with_options(options) {
                    Ok(client)=> client,
                    Err(err) => panic!("Erro ao recuperar conexao {:?}", err),
                }
            },
            Err(err) => {
                panic!("Erro ao parsear a string de conexao");
            }
        }
    }

    pub struct EnvironmentRepository {
        pub name: String,
        pub status: bool,
        pub value: String
    }

    #[async_trait]
    pub trait ActionsDB {
        fn new( env: Self) -> Self ;
        async fn insert_variable(&self) -> Result<(), &str>;
        async fn get_all_variables() -> Result<Vec<EnvironmentRepository>, String>;
        async fn get_variable(name :String) -> Result<Vec<EnvironmentRepository>, String>;
    }


    #[async_trait]
    impl ActionsDB for EnvironmentRepository {
        fn new(env: Self) -> Self {
            env
        }

        async fn insert_variable(&self) -> Result<(), &str> {
            let client_mongo = get_client_mongo().await;
            let db = client_mongo.database("environments_collections");
            let collection = db.collection("environments");

            let document = doc! {
                "name": &self.name,
                "value": &self.value,
                "status":&self.status
            };


            match collection.insert_one(document, None).await {
                Ok(_) => Ok(()),
                Err(_)=> Err("Erro ao inserir ")
            }


        }

        async fn get_all_variables() -> Result<Vec<EnvironmentRepository>, String> {
            unimplemented!()
        }

        async fn get_variable(name: String) -> Result<Vec<EnvironmentRepository>, String> {
            unimplemented!()
        }
    }
}