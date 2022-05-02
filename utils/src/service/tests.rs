use super::*;
use crate::config::*;
use figment::Profile;
use uuid::Variant;

mod register {
    use super::*;
    use anyhow::anyhow;

    #[tokio::test]
    async fn it_should_return_new_id_when_registration_is_ok() {
        let mut svc = before_each().await;
        svc.credential_repo
            .expect_insert_credentials_tx()
            .once()
            .return_once(|_, _| Ok(1_i64));

        let (username, password) = ("a".into(), "b".into());
        let credentials = Credentials { username, password };
        let actual = svc.register(credentials).await.unwrap();
        assert!(actual > 0);
    }

    #[tokio::test]
    async fn it_should_return_proper_error_when_registration_fails() {
        let mut svc = before_each().await;
        svc.credential_repo
            .expect_insert_credentials_tx()
            .once()
            .return_once(|_, _| Err(Error::Other(anyhow!("oops!"))));

        let (username, password) = ("a".into(), "b".into());
        let credentials = Credentials { username, password };
        let actual = svc.register(credentials).await;

        match actual {
            Err(Error::Other(_)) => (/* love my job */),
            Err(e) => panic!("unexpected error: {:?}", e),
            Ok(id) => panic!("unexpected id: {}", id),
        }
    }
}

mod login {
    use super::*;

    #[tokio::test]
    async fn it_should_return_uuid_token_when_login_is_ok() {
        let mut svc = before_each().await;
        svc.credential_repo
            .expect_check_credentials_db()
            .once()
            .return_once(|_, _| Ok(true));

        svc.token_repo.expect_save_token().once().return_once(|_, _| Ok(()));

        let (username, password) = ("a".into(), "b".into());
        let credentials = Credentials { username, password };

        let actual = svc.login(credentials).await.unwrap();
        let actual = Uuid::parse_str(&actual).unwrap();
        assert_eq!(actual.get_variant(), Variant::RFC4122);
    }

    #[tokio::test]
    async fn it_should_not_return_uuid_token_when_login_fails() {
        let mut svc = before_each().await;
        svc.credential_repo
            .expect_check_credentials_db()
            .once()
            .return_once(|_, _| Ok(false));

        svc.token_repo.expect_save_token().never();

        let (username, password) = ("a".into(), "b".into());
        let credentials = Credentials { username, password };

        let actual = svc.login(credentials).await;

        match actual {
            Err(Error::BadCredentials) => (/* life is good */),
            Err(e) => panic!("unexpected error: {:?}", e),
            Ok(token) => panic!("unexpected token: {}", token),
        }
    }
}

// aux -----

async fn before_each() -> AuthService<MockCredentialRepoApi, MockTokenRepoApi> {
    AuthService::<MockCredentialRepoApi, MockTokenRepoApi>::new().await
}

impl AuthService<MockCredentialRepoApi, MockTokenRepoApi> {
    // TODO AuthService unit tests connect to DB and trigger empty TXs, since
    // actual queries are mocked out, but ideally they shouldn't need a DB.
    async fn new() -> Self {
        let db = connect().await;
        let credential_repo = MockCredentialRepoApi::new();
        let token_repo = MockTokenRepoApi::new();
        Self {
            db,
            credential_repo,
            token_repo,
        }
    }
}

async fn connect() -> Pool<Postgres> {
    let test_profile = Profile::const_new("test");
    let cfg = Config::load_for(test_profile).unwrap();
    db::connect(&cfg).await.unwrap()
}
