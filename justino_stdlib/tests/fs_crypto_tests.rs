use justino_stdlib::crypto::{CryptoHash, JwtEngine};
use justino_stdlib::error::StdlibError;
use justino_stdlib::fs::{AsyncFile, EnvReader};

#[test]
fn test_async_file_io_and_path_traversal() -> Result<(), StdlibError> {
    let test_file = "target/test_file.txt";
    let content = "Hello from Justino Stdlib Async FS!";

    AsyncFile::write_file(test_file, content)?;
    let read_back = AsyncFile::read_file(test_file)?;
    assert_eq!(read_back, content);

    // Assert Path Traversal Violation protection
    let traversal_res = AsyncFile::read_file("../secret.txt");
    assert!(traversal_res.is_err());
    Ok(())
}

#[test]
fn test_env_reader_parsing() -> Result<(), StdlibError> {
    let env_file = "target/test.env";
    let env_content = "PORT=8080\nDB_PATH=app.db\nSECRET=\"my_secret_key\"\n";

    AsyncFile::write_file(env_file, env_content)?;
    let env_map = EnvReader::parse_env_file(env_file)?;

    assert_eq!(env_map.get("PORT").unwrap(), "8080");
    assert_eq!(env_map.get("DB_PATH").unwrap(), "app.db");
    assert_eq!(env_map.get("SECRET").unwrap(), "my_secret_key");
    Ok(())
}

#[test]
fn test_crypto_hashing_and_jwt() -> Result<(), StdlibError> {
    let password = "SuperSecretPassword123";
    let hash = CryptoHash::hash_password(password);
    assert!(CryptoHash::verify_password(password, &hash));
    assert!(!CryptoHash::verify_password("WrongPassword", &hash));

    let secret = "jwt_shared_secret";
    let payload = r#"{"user_id":42,"role":"admin"}"#;
    let token = JwtEngine::sign(payload, secret);

    let decoded = JwtEngine::verify(&token, secret)?;
    assert_eq!(decoded, payload);
    Ok(())
}
