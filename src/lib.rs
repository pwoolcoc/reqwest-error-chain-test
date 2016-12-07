extern crate reqwest;
#[macro_use] extern crate error_chain;

mod errors {
    error_chain!{
        foreign_links {
            ReqwestError(::reqwest::Error);
        }
    }    
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
