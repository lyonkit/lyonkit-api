#[derive(Debug)]
enum Error {
    ReqError(minreq::Error),
    StatusCodeNot200(i32),
}

impl From<minreq::Error> for Error {
    fn from(e: minreq::Error) -> Self {
        Error::ReqError(e)
    }
}

fn main() -> Result<(), Error> {
    let res = minreq::get(format!(
        "http://{}:{}/api/ping",
        std::env::var("HC_HOST").unwrap_or_else(|_| "localhost".to_string()),
        std::env::var("HC_PORT").unwrap_or_else(|_| "8080".to_string())
    ))
    .send()?;

    if res.status_code == 200 {
        println!("{}", res.as_str()?);
        Ok(())
    } else {
        Err(Error::StatusCodeNot200(res.status_code))
    }
}
