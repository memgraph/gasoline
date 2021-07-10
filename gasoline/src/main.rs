use gasoline::{Connection, GasolineError, Memgraph};

fn execute_query() -> Result<(), GasolineError> {
    let mut memgraph = Memgraph {
        host: Some("localhost".to_string()),
        port: 7687,
        username: Some("".to_string()),
        password: Some("".to_string()),
        connection: None,
        encrypted: false,
    };
    memgraph.connect();
    let data = memgraph.execute_and_fetch("MATCH (n) RETURN n.prop;")?;
    println!("{}", data.len());
    Ok(())
}

fn main() {
    if let Err(_) = execute_query() {
        panic!("Gasoline error");
    }
}
