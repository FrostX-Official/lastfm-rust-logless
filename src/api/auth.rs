mod get_session;
mod get_token;

use crate::Lastfm;
pub use get_session::AuthGetSession;
pub use get_token::AuthGetToken;

#[derive(Debug)]
pub struct Auth<'a> {
    lastfm: &'a Lastfm,
    api_key: String,
}

impl<'a> Auth<'a> {
    pub(crate) fn new(lastfm: &'a Lastfm) -> Self {
        Self {
            lastfm,
            api_key: lastfm.get_api_key(),
        }
    }

    pub fn get_token(&mut self) -> AuthGetToken<'_> {
        AuthGetToken::new(self.lastfm)
    }

    pub fn get_session(&mut self) -> AuthGetSession<'_> {
        AuthGetSession::new(self.lastfm)
    }

    pub fn get_mobile_session(&mut self) -> AuthGetSession<'_> {
        //TODO: get mobile session
        todo!()
    }

    pub fn pls_authorize(&self, token: String) {
        println!("{token}");
        let auth_url = format!(
            "https://www.last.fm/api/auth?api_key={}&token={}",
            self.api_key,
            token.replace("\"", "")
        );

        println!("Please authorize the application by visiting the following URL:");
        println!("{}", auth_url);
        println!("\nOnce you have authorized the app, press Enter to continue...");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        println!("Thank you! Continuing...");
    }
}
