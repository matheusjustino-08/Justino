use justino_core::JustinoError;
use justino_ide::auth::OAuthClient;

#[test]
fn test_oauth_pkce_login_url_generation() {
    let auth = OAuthClient::new();
    let (url, verifier) = auth.generate_pkce_login_url();

    assert!(url.contains("response_type=code"));
    assert!(url.contains("justino-ide://oauth/callback"));
    assert_eq!(verifier, "JUSTINO_PKCE_VERIFIER_SECRET_123456789");
}

#[test]
fn test_oauth_token_exchange() -> Result<(), JustinoError> {
    let auth = OAuthClient::new();
    let session = auth.exchange_code_for_token("auth_code_123456")?;

    assert!(session.is_authenticated);
    assert_eq!(session.username, "Justino Developer");
    assert!(!session.jwt_token.is_empty());
    Ok(())
}
