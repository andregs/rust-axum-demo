use super::*;
use crate::config::{context::redis, Config, Profile};
use uuid::Uuid;

#[tokio::test]
async fn test_save_and_check() {
    let repo = before_each();
    let token = Uuid::new_v4().to_string();
    let username = "username".to_string();

    match repo.get_username(&token).await {
        Err(Error::BadToken) => (/* good */),
        Err(e) => panic!("unexpected error: {:?}", e),
        Ok(username) => panic!("unexpected username: {}", username),
    }

    repo.save_token(&token, &username).await.unwrap();
    let actual = repo.get_username(&token).await.expect("username was expected");

    assert_eq!(actual, username);
}

// aux ----

fn before_each() -> RedisTokenRepo {
    let test_profile = Profile::const_new("test");
    let cfg = Config::load_for(test_profile).unwrap();
    let client = redis::open(&cfg).unwrap();
    RedisTokenRepo::new(&client)
}
