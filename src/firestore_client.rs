use std::str::FromStr;

use crate::Error;
use crate::Precondition;

use serde_firestore_value::google;
use serde_firestore_value::google::firestore::v1::ExecutePipelineRequest;
use serde_firestore_value::google::firestore::v1::ExecutePipelineResponse;

#[derive(Debug, thiserror::Error)]
enum E {
    #[error("auth error: {0}")]
    Auth(#[from] google_cloud_auth::errors::CredentialsError),
    #[error("build auth error: {0}")]
    BuildAuth(#[from] google_cloud_auth::build_errors::Error),
    #[error("invalid url")]
    InvalidUrl(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("transport error: {0}")]
    Transport(#[from] tonic::transport::Error),
    #[error("status error: {0}")]
    Status(#[from] tonic::Status),
}

impl From<E> for Error {
    fn from(e: E) -> Self {
        Self::from_source(Box::new(e))
    }
}

#[derive(Clone)]
pub(crate) struct FirestoreClient {
    channel: tonic::transport::Channel,
    credentials: Option<google_cloud_auth::credentials::Credentials>,
    database_name: firestore_path::DatabaseName,
}

impl FirestoreClient {
    // NOTE: No tests are written for this method (requires a real project).
    pub(crate) fn new(database: String, emulator_host: Option<String>) -> Result<Self, Error> {
        let (channel, credentials) = match emulator_host {
            Some(host) => (
                tonic::transport::Channel::from_shared(format!("http://{}", host))
                    .map_err(|e| E::InvalidUrl(Box::new(e)))?
                    .connect_lazy(),
                None,
            ),
            None => (
                tonic::transport::Channel::from_static("https://firestore.googleapis.com")
                    .tls_config(
                        tonic::transport::ClientTlsConfig::new()
                            .domain_name("firestore.googleapis.com")
                            .with_webpki_roots(),
                    )
                    .map_err(E::from)?
                    .connect_lazy(),
                Some(
                    google_cloud_auth::credentials::Builder::default()
                        .with_scopes(["https://www.googleapis.com/auth/datastore"])
                        .build()
                        .map_err(E::from)?,
                ),
            ),
        };
        let database_name =
            <firestore_path::DatabaseName as std::str::FromStr>::from_str(&database).unwrap();
        Ok(Self {
            channel,
            credentials,
            database_name,
        })
    }

    // NOTE: No tests are written for this method (requires a real project).
    #[allow(dead_code)]
    pub(crate) async fn execute_pipeline(
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

    pub(crate) async fn create_document(
        &self,
        document_path: &firestore_path::DocumentPath,
        value: google::firestore::v1::Value,
    ) -> Result<::prost_types::Timestamp, Error> {
        let mut client = self.client().await?;
        let request = google::firestore::v1::CommitRequest {
            database: self.database_name.to_string(),
            writes: vec![google::firestore::v1::Write {
                update_mask: None,
                update_transforms: vec![],
                current_document: Some(google::firestore::v1::Precondition {
                    condition_type: Some(
                        google::firestore::v1::precondition::ConditionType::Exists(false),
                    ),
                }),
                operation: Some(google::firestore::v1::write::Operation::Update(
                    google::firestore::v1::Document {
                        name: self
                            .database_name
                            .doc(document_path.to_string())
                            .expect("invalid document path")
                            .to_string(),
                        fields: {
                            match value.value_type {
                                Some(google::firestore::v1::value::ValueType::MapValue(
                                    map_value,
                                )) => map_value.fields,
                                _ => {
                                    return Err(Error::from_source("value must be a map".into()));
                                }
                            }
                        },
                        create_time: None,
                        update_time: None,
                    },
                )),
            }],
            transaction: vec![],
        };
        let response = client
            .commit(request)
            .await
            .map_err(|e| Error::from_source(Box::new(e)))?;
        let commit_response = response.into_inner();
        let write_result = commit_response.write_results.into_iter().next().unwrap();
        Ok(write_result.update_time.unwrap_or_else(|| {
            commit_response
                .commit_time
                .expect("commit_time should be set")
        }))
    }

    pub(crate) async fn delete_document(
        &self,
        document_path: &firestore_path::DocumentPath,
        precondition: Precondition,
    ) -> Result<::prost_types::Timestamp, Error> {
        let mut client = self.client().await?;
        let request =
            google::firestore::v1::CommitRequest {
                database: self.database_name.to_string(),
                writes: vec![google::firestore::v1::Write {
                    update_mask: None,
                    update_transforms: vec![],
                    current_document: {
                        let Precondition {
                            exists,
                            last_update_time,
                        } = precondition;
                        match (exists, last_update_time) {
                        (None, None) => None,
                        (Some(exists), None) => Some(google::firestore::v1::Precondition {
                            condition_type: Some(
                                google::firestore::v1::precondition::ConditionType::Exists(exists),
                            ),
                        }),
                        (None, Some(last_update_time)) => Some(google::firestore::v1::Precondition {
                            condition_type: Some(
                                google::firestore::v1::precondition::ConditionType::UpdateTime(
                                    last_update_time.into_prost_timestamp(),
                                ),
                            ),
                        }),
                        (Some(_), Some(_)) => {
                            return Err(Error::from_source(
                                "both exists and last_update_time are set".into(),
                            ));
                        }
                    }
                    },
                    operation: Some(google::firestore::v1::write::Operation::Delete(
                        self.database_name
                            .doc(document_path.to_string())
                            .expect("invalid document path")
                            .to_string(),
                    )),
                }],
                transaction: vec![],
            };
        let response = client
            .commit(request)
            .await
            .map_err(|e| Error::from_source(Box::new(e)))?;
        let commit_response = response.into_inner();
        let write_result = commit_response.write_results.into_iter().next().unwrap();
        Ok(write_result.update_time.unwrap_or_else(|| {
            commit_response
                .commit_time
                .expect("commit_time should be set")
        }))
    }

    pub(crate) async fn get_document(
        &self,
        document_path: &firestore_path::DocumentPath,
    ) -> Result<Option<google::firestore::v1::Document>, Error> {
        let mut client = self.client().await?;
        let request = google::firestore::v1::GetDocumentRequest {
            name: self
                .database_name
                .doc(document_path.to_string())
                .expect("invalid document path")
                .to_string(),
            mask: None,
            consistency_selector: None,
        };
        let result = client.get_document(request).await;
        match result {
            Ok(response) => Ok(Some(response.into_inner())),
            Err(status) => match status.code() {
                tonic::Code::NotFound => Ok(None),
                _ => Err(Error::from(E::from(status))),
            },
        }
    }

    pub(crate) async fn list_documents(
        &self,
        collection_path: &firestore_path::CollectionPath,
    ) -> Result<Vec<firestore_path::DocumentPath>, Error> {
        let root_document_name = self.database_name.root_document_name().to_string();
        let parent = match collection_path.parent() {
            Some(parent_doc_path) => self
                .database_name
                .doc(firestore_path::DocumentPath::from_str(&parent_doc_path.to_string()).unwrap())
                .unwrap()
                .to_string(),
            None => root_document_name.clone(),
        };
        let collection_id = collection_path.collection_id().to_string();
        let mut result = Vec::new();
        let mut page_token = String::new();
        loop {
            let mut client = self.client().await?;
            let request = google::firestore::v1::ListDocumentsRequest {
                parent: parent.clone(),
                collection_id: collection_id.clone(),
                // use server default page size
                page_size: 0,
                page_token: page_token.clone(),
                // __name__ ASC
                order_by: String::new(),
                mask: Some(google::firestore::v1::DocumentMask {
                    field_paths: vec![],
                }),
                show_missing: true,
                consistency_selector: None,
            };
            let response = client.list_documents(request).await.map_err(E::from)?;
            let list_response = response.into_inner();
            result.extend(
                list_response
                    .documents
                    .into_iter()
                    .map(
                        |doc| -> Result<firestore_path::DocumentPath, firestore_path::Error> {
                            Ok(firestore_path::DocumentPath::from(
                                firestore_path::DocumentName::from_str(&doc.name)?,
                            ))
                        },
                    )
                    .collect::<Result<Vec<_>, firestore_path::Error>>()
                    .map_err(|e| Error::from_source(Box::new(e)))?,
            );
            page_token = list_response.next_page_token;
            if page_token.is_empty() {
                break;
            }
        }
        Ok(result)
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
        let header_map = match self.credentials {
            None => {
                let mut header_map = http::HeaderMap::new();
                header_map.insert(
                    http::header::AUTHORIZATION,
                    http::HeaderValue::from_static("Bearer owner"),
                );
                header_map
            }
            Some(ref credentials) => {
                let cacheable_headers = credentials.headers(http::Extensions::new()).await?;
                match cacheable_headers {
                    google_cloud_auth::credentials::CacheableResource::New { data, .. } => data,
                    google_cloud_auth::credentials::CacheableResource::NotModified => {
                        todo!()
                    }
                }
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
        let mut client = FirestoreClient::new(database_name.to_string(), None)?;
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
        let mut client = FirestoreClient::new(database_name.to_string(), None)?;
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
