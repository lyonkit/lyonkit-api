use crate::{services::locale::LocaleFixtures, test_app::TestApp};
use serde_json::json;
use test_context::test_context;

#[test_context(TestApp)]
#[tokio::test]
async fn update_locale_should_work(ctx: &mut TestApp) {
    let namespace = "test_update_locale";
    ctx.create_api_key(namespace, false).await;

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

    let update_result = ctx
        .put(
            "/locale/fr",
            json!({"key1":"updated_value1", "key2":"value2"}),
        )
        .await
        .json::<serde_json::Value>()
        .await
        .expect("Failed to parse json");

    assert_eq!(
        &json!({"key1":"updated_value1", "key2":"value2"}),
        update_result.get("messages").unwrap()
    );
    assert_eq!(&json!(namespace), update_result.get("namespace").unwrap());
    assert_eq!(&json!("fr"), update_result.get("lang").unwrap());
}
