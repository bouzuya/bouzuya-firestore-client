use std::str::FromStr;

use crate::Error;

use serde_firestore_value::google;
use serde_firestore_value::google::firestore::v1::ExecutePipelineRequest;
use serde_firestore_value::google::firestore::v1::ExecutePipelineResponse;

use crate::E;

pub struct FirestoreClient {
    channel: tonic::transport::Channel,
    credentials: google_cloud_auth::credentials::Credentials,
    database_name: firestore_path::DatabaseName,
}

impl FirestoreClient {
    pub async fn new(database: String) -> Result<Self, Error> {
        let credentials = google_cloud_auth::credentials::Builder::default()
            .with_scopes(["https://www.googleapis.com/auth/datastore"])
            .build()
            .map_err(E::from)?;
        let channel = tonic::transport::Channel::from_static("https://firestore.googleapis.com")
            .tls_config(
                tonic::transport::ClientTlsConfig::new()
                    .domain_name("firestore.googleapis.com")
                    .with_webpki_roots(),
            )
            .map_err(E::from)?
            .connect()
            .await
            .map_err(E::from)?;
        let database_name =
            <firestore_path::DatabaseName as std::str::FromStr>::from_str(&database).unwrap();
        Ok(Self {
            channel,
            credentials,
            database_name,
        })
    }

    pub async fn execute_pipeline(
        &mut self,
        request: ExecutePipelineRequest,
    ) -> Result<tonic::Response<tonic::codec::Streaming<ExecutePipelineResponse>>, Error> {
        let mut client = self.client().await?;
        let mut request = tonic::Request::new(request);
        request.metadata_mut().append(
            "x-goog-request-params",
            // It causes an error if the order is database_id, project_id
            tonic::metadata::MetadataValue::from_str(&format!(
                "project_id={}&database_id={}",
                self.database_name.project_id(),
                self.database_name.database_id(),
            ))
            .unwrap(),
        );
        let response = client.execute_pipeline(request).await.map_err(E::from)?;
        Ok(response)
    }

    async fn client(
        &self,
    ) -> Result<
        google::firestore::v1::firestore_client::FirestoreClient<
            tonic::service::interceptor::InterceptedService<
                tonic::transport::Channel,
                impl FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
            >,
        >,
        E,
    > {
        let cacheable_headers = self.credentials.headers(http::Extensions::new()).await?;
        let header_map = match cacheable_headers {
            google_cloud_auth::credentials::CacheableResource::New { data, .. } => data,
            google_cloud_auth::credentials::CacheableResource::NotModified => {
                todo!()
            }
        };
        let metadata = tonic::metadata::MetadataMap::from_headers(header_map);
        let firestore_client =
            google::firestore::v1::firestore_client::FirestoreClient::with_interceptor(
                self.channel.clone(),
                move |mut request: tonic::Request<()>| {
                    for key_and_value in metadata.iter() {
                        match key_and_value {
                            tonic::metadata::KeyAndValueRef::Ascii(key, value) => {
                                request.metadata_mut().insert(key, value.clone());
                            }
                            tonic::metadata::KeyAndValueRef::Binary(key, value) => {
                                request.metadata_mut().insert_bin(key, value.clone());
                            }
                        }
                    }
                    Ok(request)
                },
            );

        Ok(firestore_client)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[ignore = "real project required"]
    #[tokio::test]
    async fn test_execute_pipeline() -> anyhow::Result<()> {
        let project_id = std::env::var("PROJECT_ID")?;
        let database_id = std::env::var("DATABASE_ID")?;
        let database_name = firestore_path::DatabaseName::new(
            firestore_path::ProjectId::from_str(&project_id)?,
            firestore_path::DatabaseId::from_str(&database_id)?,
        );
        let mut client = FirestoreClient::new(database_name.to_string()).await?;
        let request = google::firestore::v1::ExecutePipelineRequest {
            database: database_name.to_string(),
            pipeline_type: Some(
                google::firestore::v1::execute_pipeline_request::PipelineType::StructuredPipeline(
                    google::firestore::v1::StructuredPipeline {
                        pipeline: Some(google::firestore::v1::Pipeline {
                            stages: vec![google::firestore::v1::pipeline::Stage {
                                name: "collection".to_string(),
                                args: vec![google::firestore::v1::Value {
                                    value_type: Some(
                                        google::firestore::v1::value::ValueType::ReferenceValue(
                                            "/users".to_string(),
                                        ),
                                    ),
                                }],
                                options: std::collections::HashMap::new(),
                            }],
                        }),
                        options: std::collections::HashMap::new(),
                    },
                ),
            ),
            consistency_selector: None,
        };
        assert!(client.execute_pipeline(request).await.is_ok());
        // let mut streaming = client.execute_pipeline(request).await?.into_inner();
        // while let Some(response) = streaming.message().await? {
        //     println!("{:#?}", response);
        // }
        Ok(())
    }

    #[ignore = "real project required"]
    #[tokio::test]
    async fn test_execute_pipeline_2() -> anyhow::Result<()> {
        let project_id = std::env::var("PROJECT_ID")?;
        let database_id = std::env::var("DATABASE_ID")?;
        let database_name = firestore_path::DatabaseName::new(
            firestore_path::ProjectId::from_str(&project_id)?,
            firestore_path::DatabaseId::from_str(&database_id)?,
        );
        let mut client = FirestoreClient::new(database_name.to_string()).await?;
        let request = google::firestore::v1::ExecutePipelineRequest {
            database: database_name.to_string(),
            pipeline_type: Some(
                google::firestore::v1::execute_pipeline_request::PipelineType::StructuredPipeline(
                    google::firestore::v1::StructuredPipeline {
                        pipeline: Some(google::firestore::v1::Pipeline {
                            stages: vec![
                                google::firestore::v1::pipeline::Stage {
                                    name: "collection".to_string(),
                                    args: vec![google::firestore::v1::Value {
                                        value_type: Some(
                                            google::firestore::v1::value::ValueType::ReferenceValue(
                                                "/users".to_string(),
                                            ),
                                        ),
                                    }],
                                    options: std::collections::HashMap::new(),
                                },
                                google::firestore::v1::pipeline::Stage {
                                    name: "where".to_string(),
                                    args: vec![google::firestore::v1::Value {
                                        value_type: Some(google::firestore::v1::value::ValueType::FunctionValue(
                                            google::firestore::v1::Function {
                                            name: "greater_than_or_equal".to_string(),
                                            args: vec![
                                                google::firestore::v1::Value {
                                                    value_type: Some(
                                                        google::firestore::v1::value::ValueType::FieldReferenceValue(
                                                            "age".to_string(),
                                                        ),
                                                    ),
                                                },
                                                google::firestore::v1::Value {
                                                    value_type: Some(
                                                        google::firestore::v1::value::ValueType::IntegerValue(
                                                            36,
                                                        ),
                                                    ),
                                                },
                                            ],
                                            options: std::collections::HashMap::new(),
                                        })),
                                    }],
                                    options: std::collections::HashMap::new(),
                                },
                            ],
                        }),
                        options: std::collections::HashMap::new(),
                    },
                ),
            ),
            consistency_selector: None,
        };
        assert!(client.execute_pipeline(request).await.is_ok());
        // let mut streaming = client.execute_pipeline(request).await?.into_inner();
        // while let Some(response) = streaming.message().await? {
        //     println!("{:#?}", response);
        // }
        Ok(())
    }
}
