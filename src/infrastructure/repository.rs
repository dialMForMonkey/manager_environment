
#[cfg(test)]
mod test_integration {
    use crate::infrastructure::repository::manager_environment_repository;
    use crate::infrastructure::repository::manager_environment_repository::ActionsDB;
    use tokio;
    use std::env;
    use dotenv::dotenv;
    use mongodb::options::ClientOptions;
    use mongodb::Client;


    #[tokio::test]
    async fn insert_variable_test(){
        dotenv::from_filename("test_integration.env").unwrap();

       let env:  manager_environment_repository::EnvironmentRepository =   manager_environment_repository::EnvironmentRepository {
            name: "teste".to_string(),
            status:false,
            value: "teste".to_string()
       };
        let actions = manager_environment_repository::ActionsDB::new(env);

        let result_insert = actions.insert_variable().await;

        assert!(result_insert.is_ok())

    }

    #[tokio::test]
    async fn get_all_variable_test(){
        dotenv::from_filename("test_integration.env").unwrap();

        let env:  manager_environment_repository::EnvironmentRepository =   manager_environment_repository::EnvironmentRepository {
            name: "teste".to_string(),
            status:false,
            value: "teste".to_string()
        };
        //ver a necessidade disso quando a funcao no precisa do objeto
        let  manager= manager_environment_repository::ActionsDB::new(env);

        let r_variables = manager.get_all_variables().await;
        assert!(r_variables.is_ok());
        let variables  = r_variables.unwrap();
        assert_ne!(variables.len(), 0)

    }

    #[tokio::test]
    async fn get_variable_test(){
        dotenv::from_filename("test_integration.env").unwrap();
        let env:  manager_environment_repository::EnvironmentRepository =   manager_environment_repository::EnvironmentRepository {
            name: "teste".to_string(),
            status:false,
            value: "teste".to_string()
        };
        //ver a necessidade disso quando a funcao no precisa do objeto
        let  manager= manager_environment_repository::ActionsDB::new(env);

        let r_variables = manager.get_variable("teste").await;

        assert_eq!(1, 0)
    }

}



pub mod manager_environment_repository {
    use crate::actor_models::environment_variable_actor::Variable;
    use async_trait::async_trait;
    use mongodb::{Client, options::ClientOptions, Collection};
    use mongodb::bson::doc;
    use tokio::stream::StreamExt;
    use bson::Bson;
    use bson::spec::ElementType::Boolean;


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
        async fn get_all_variables(&self) -> Result<Vec<EnvironmentRepository>, &str>;
        async fn get_variable(&self, name: &str) -> Result<Vec<EnvironmentRepository>, &'static str>;
    }


    async fn get_client_collection(db_name: &str, collection_name: &str) -> Collection {
        let client_mongo = get_client_mongo().await;
        let db = client_mongo.database(db_name);
        db.collection(collection_name )
    }

    #[async_trait]
    impl ActionsDB for EnvironmentRepository {
        fn new(env: Self) -> Self {
            env
        }


        async fn insert_variable(&self) -> Result<(), &str> {
            let collection= get_client_collection("environments_collections", "environments").await;

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

        async fn get_all_variables(&self) -> Result<Vec<EnvironmentRepository>, &str> {
            let collection = get_client_collection("environments_collections", "environments").await;

            let result_cursor = collection.find(None, None).await;
            match result_cursor {
                Ok(cursor) => {
                    Ok(
                        cursor.map(|cursor |{
                            let document: bson::document::Document  = cursor.unwrap();

                            EnvironmentRepository {
                                name: match document.get("name"){
                                    Some(name_bson) => {
                                        name_bson
                                            .as_str()
                                            .unwrap_or("")
                                            .parse()
                                            .unwrap_or(String::new())
                                    },
                                    None => String::new()
                                },
                                status: match document.get("status") {
                                    Some(status_bson) => {
                                        status_bson
                                            .as_bool()
                                            .unwrap_or(false)
                                    },
                                    None => false
                                },
                                value: match document.get("value") {
                                    Some(value_bson) => {
                                        value_bson
                                            .as_str()
                                            .unwrap_or("")
                                            .parse()
                                            .unwrap_or(String::new())
                                    },
                                    None=> String::new()
                                }
                            }
                        })
                            .collect::<Vec<EnvironmentRepository>>().await

                    )
                },
                Err(_) => {
                    Err("Erro ao buscar find")
                }
            }
        }

        async fn get_variable(&self ,name: &str) -> Result<Vec<EnvironmentRepository>, &'static str> {
            let collection = get_client_collection("environments_collections", "environments").await;

            let document_for_find = doc! {
                "name": name
            };


            let result_cursor = collection.find(Some(document_for_find), None).await.unwrap();
            println!("{:?}", result_cursor);

            Err("erro")
        }
    }
}