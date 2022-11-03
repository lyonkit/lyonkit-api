use crate::services::locale::LocaleFixtures;
use crate::test_app::TestApp;
use serde_json::json;
use test_context::test_context;

#[test_context(TestApp)]
#[tokio::test]
async fn get_locales_should_work(ctx: &mut TestApp) {
    let namespace = "test_locales";
    ctx.create_api_key(namespace, true).await;
    ctx.database_connection()
        .create_locale(namespace, "fr", json!({"key1":"value1", "key2":"value2"}))
        .await;
    ctx.database_connection()
        .create_locale(
            namespace,
            "en",
            json!({"key1":"value1_en", "key2":"value2_en"}),
        )
        .await;

    let res = ctx
        .get("/locale")
        .await
        .json::<serde_json::Value>()
        .await
        .expect("Failed to parse response");

    assert_eq!(
        json!({"fr":{"key1":"value1", "key2":"value2"}, "en":{"key1":"value1_en", "key2":"value2_en"}}),
        res
    );
}
