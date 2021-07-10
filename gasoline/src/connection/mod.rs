use crate::model::Value;

pub enum GasolineError {
    ConnectFailure,
    ExecuteAndFetchError,
}

pub trait Connection {
    fn execute(&self, query: &str) -> Result<(), GasolineError>;
    fn execute_and_fetch(
        &mut self,
        query: &str,
    ) -> Result<std::vec::Vec<std::vec::Vec<Value>>, GasolineError>;
    fn connect<'a>(&'a mut self) -> &'a Self;
}

pub struct Memgraph {
    pub host: Option<String>,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
    pub encrypted: bool,
    pub connection: Option<rsmgclient::Connection>,
}

impl Connection for Memgraph {
    fn execute(&self, _query: &str) -> Result<(), GasolineError> {
        Ok(())
    }

    fn execute_and_fetch(
        &mut self,
        query: &str,
    ) -> Result<std::vec::Vec<std::vec::Vec<Value>>, GasolineError> {
        match self.connection.as_mut().unwrap().execute(query, None) {
            Ok(_) => {}
            Err(_) => return Err(GasolineError::ExecuteAndFetchError),
        }
        let mut result_vec = Vec::new();
        for record in match self.connection.as_mut().unwrap().fetchall() {
            Ok(x) => x,
            Err(_) => return Err(GasolineError::ExecuteAndFetchError),
        } {
            let mut record_vec = Vec::new();
            for value in record.values {
                record_vec.push(match value {
                    rsmgclient::Value::Null => Value::Null,
                    rsmgclient::Value::Bool(x) => Value::Bool(x),
                    rsmgclient::Value::Int(x) => Value::Int(x),
                    rsmgclient::Value::Float(x) => Value::Float(x),
                    rsmgclient::Value::String(x) => Value::String(x),
                    _ => panic!("Not implemented yet"),
                });
            }
            result_vec.push(record_vec);
        }
        match self.connection.as_mut().unwrap().commit() {
            Ok(_) => Ok(result_vec),
            Err(_) => Err(GasolineError::ExecuteAndFetchError),
        }
    }

    fn connect<'a>(&'a mut self) -> &'a Self {
        let connect_params = rsmgclient::ConnectParams {
            host: self.host.clone(),
            port: self.port,
            username: self.username.clone(),
            password: self.password.clone(),
            sslmode: if self.encrypted {
                rsmgclient::SSLMode::Require
            } else {
                rsmgclient::SSLMode::Disable
            },
            ..Default::default()
        };
        match rsmgclient::Connection::connect(&connect_params) {
            Ok(c) => {
                self.connection = Some(c);
                self
            }
            Err(_) => panic!("Connection failure!"),
        }
    }
}
