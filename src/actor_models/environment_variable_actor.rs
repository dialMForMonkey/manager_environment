/**
    nome da variavel
	status
	valor
**/

use actix::prelude::*;

pub struct Variable {
    pub name: String,
    pub situation:  Option<bool>, // on off
    pub value: Option<String>
}

#[derive(Message)]
#[rtype(result = "Result<bool, ()>")]
struct VariableSituationMessage {
    situation: bool
}

#[derive(Message)]
#[rtype(result = "Result<bool, ()>")]
struct VariableCreateMessage {
    name: String,
    value:String
}

impl Actor for Variable {
  type Context = Context<Self>;
}

impl Handler<VariableSituationMessage> for Variable {
    type Result = Result<bool, ()>;

    fn handle(&mut self, msg: VariableSituationMessage, _ctx: &mut Context<Self>) -> Self::Result {
        Ok(true)
    }
}


impl Handler<VariableCreateMessage> for Variable {
    type Result = Result<bool, ()>;

    fn handle(&mut self, msg: VariableCreateMessage, _ctx: &mut Context<Self>) -> Self::Result {
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;


    #[actix_rt::test]
    async fn test_send_message_situation_message() {

        let variable = Variable::start(
            Variable {
                    name: "lero".to_string(),
                    situation:None,
                    value:None
            });

        let  response = variable.send(VariableSituationMessage{ situation:false }).await;
        assert!(response.is_ok());

        let a: Result<bool, ()> = response.unwrap();

        assert!(a.is_ok());
        assert!(a.unwrap());

    }


}