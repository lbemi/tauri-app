use k8s_openapi::api::core::v1::Pod;
use kube::api::{DeleteParams, ListParams, PostParams};
use kube::{Api, Client};

use crate::error::MyError;

pub struct PodStruct {
    api: Api<Pod>,
}

impl PodStruct {
    pub async fn namespace(ns: &str) -> Self {
        let client = Client::try_default().await.unwrap();
        if ns != "all" {
            let api: Api<Pod> = Api::namespaced(client, ns);
            Self { api }
        } else {
            let api: Api<Pod> = Api::all(client);
            Self { api }
        }
    }

    pub async fn list_pods(&self) -> Result<Vec<Pod>, MyError> {
        let res = self.api.list(&ListParams::default()).await?;
        Ok(res.items)
    }

    pub async fn get_pod_by_name(&self, name: String) -> Result<Pod, MyError> {
        let res = self.api.get(&name).await?;
        Ok(res)
    }

    pub async fn delete_pod_by_name(&self, name: String) -> Result<String, MyError> {
        let res = self.api.delete(&name, &DeleteParams::default()).await?;
        Ok(format!("{:?}", res))
    }

    pub async fn create_pod(&self, pod: Pod) -> Result<Pod, MyError> {
        let res = self.api.create(&PostParams::default(), &pod).await?;
        Ok(res)
    }
}
