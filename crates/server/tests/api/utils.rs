use aws_sdk_s3::{
    model::{Delete, ObjectIdentifier},
    Client,
};
use futures::future::try_join_all;

/// Deletes a bucket from S3 by removing all its files first
pub async fn wipe_bucket(s3_client: &Client, s3_bucket: &String) {
    let objects = s3_client
        .list_objects_v2()
        .bucket(s3_bucket)
        .send()
        .await
        .unwrap();

    let mut delete_object_fut = Vec::new();
    let mut len = 0;
    let mut delete_keys = Delete::builder();

    for obj in objects.contents().unwrap_or_default() {
        if let Some(key) = obj.key() {
            delete_keys = delete_keys.objects(ObjectIdentifier::builder().key(key).build());
            len += 1;
            if len >= 1000 {
                delete_object_fut.push(
                    s3_client
                        .delete_objects()
                        .bucket(s3_bucket)
                        .delete(delete_keys.build())
                        .send(),
                );
                delete_keys = Delete::builder();
                len = 0;
            }
        }
    }

    if len > 0 {
        delete_object_fut.push(
            s3_client
                .delete_objects()
                .bucket(s3_bucket)
                .delete(delete_keys.build())
                .send(),
        );
    }

    try_join_all(delete_object_fut).await.ok();

    s3_client
        .delete_bucket()
        .bucket(s3_bucket)
        .send()
        .await
        .ok();
}
