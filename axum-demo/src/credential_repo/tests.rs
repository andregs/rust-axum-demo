use super::*;
use crate::config::{context::db, Config};
use figment::Profile;
use uuid::Uuid;

mod insert_credentials {
    use super::*;

    #[tokio::test]
    async fn it_should_insert_good_credentials() {
        let (mut tx, repo) = before_each().await;
        let credentials = &new_random_credentials();
        let new_id = repo.insert_credentials(&mut tx, credentials).await.unwrap();
        assert!(new_id > 0);
    }

    #[tokio::test]
    async fn it_should_reject_duplicated_username() {
        let (mut tx, repo) = before_each().await;
        let credentials = &new_random_credentials();
        let new_id = repo.insert_credentials(&mut tx, credentials).await.unwrap();
        assert!(new_id > 0);

        let result = repo.insert_credentials(&mut tx, credentials).await;
        match result {
            // TODO one day we'll have assert_matches https://github.com/rust-lang/rust/issues/82775
            Err(Error::Duplicated(_)) => (/* alright */),
            Err(e) => panic!("unexpected error: {:?}", e),
            Ok(id) => panic!("unexpected id: {:?}", id),
        }
    }

    #[tokio::test]
    async fn it_should_reject_username_too_big() {
        let (mut tx, repo) = before_each().await;
        let mut credentials = new_random_credentials();
        credentials.username = format!("{0}{0}", credentials.username);

        let result = repo.insert_credentials(&mut tx, &credentials).await;
        match result {
            Err(Error::TooBig(_)) => (/* ❤ boilerplate */),
            Err(e) => panic!("unexpected error {:?}", e),
            Ok(id) => panic!("unexpected id: {:?}", id),
        }
    }

    // TODO test of MVCC, where a tx tries to read data that has already been changed by another concurrent tx
}

mod check_credentials {
    use super::*;

    #[tokio::test]
    async fn it_should_find_valid_credentials() {
        let (mut tx, repo) = before_each().await;
        let credentials = &new_random_credentials();
        repo.insert_credentials(&mut tx, credentials).await.unwrap();

        let is_valid = repo.check_credentials_tx(&mut tx, credentials).await.unwrap();
        assert!(is_valid);
    }

    #[tokio::test]
    async fn it_should_not_find_when_username_is_wrong() {
        let (mut tx, repo) = before_each().await;
        let credentials = new_random_credentials();

        let is_valid = repo.check_credentials_tx(&mut tx, &credentials).await.unwrap();
        assert!(!is_valid);
    }

    #[tokio::test]
    async fn it_should_not_find_when_password_is_wrong() {
        let (mut tx, repo) = before_each().await;
        let mut credentials = new_random_credentials();
        repo.insert_credentials(&mut tx, &credentials).await.unwrap();
        credentials.password = String::from("wrong password");

        let is_valid = repo.check_credentials_tx(&mut tx, &credentials).await.unwrap();
        assert!(!is_valid);
    }
}

// aux ----

async fn before_each() -> (Transaction, PostgresCredentialRepo) {
    let db = connect().await;
    let tx = db.begin().await.unwrap();
    let repo = PostgresCredentialRepo;
    (tx, repo)
}

async fn connect() -> Pool<Postgres> {
    let test_profile = Profile::const_new("test");
    let cfg = Config::load_for(test_profile).unwrap();
    db::connect(&cfg).await.unwrap()
}

fn new_random_credentials() -> Credentials {
    let uuid = Uuid::new_v4().to_string();
    let username = format!("test-{}", uuid);
    let password = uuid;
    Credentials { username, password }
}
