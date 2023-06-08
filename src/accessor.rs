use crate::{
    client::Client,
    error::Error,
};

use item_configurator_server::{
    JsonGetter,
    JsonSetter,
    ItemGetter,
    ItemSetter,
    async_trait,
    TypeId,
};

use std::collections::HashMap;

#[async_trait]
impl JsonGetter for Client {
    type Error = Error;
    async fn get_json(
        &self,
        config_name: &str,
    ) -> Result<Vec<String>, Self::Error> {
        self.download_json(config_name).await
    }
}

#[async_trait]
impl JsonSetter for Client {
    type Error = Error;
    async fn set_json(
        &self,
        config_name: &str,
        json: Vec<String>,
    ) -> Result<(), Self::Error> {
        self.upload_json(json, config_name).await
    }
}

#[async_trait]
impl ItemGetter for Client {
    type Error = Error;
    async fn get_items(
        &self,
        config_name: &str,
    ) -> Result<HashMap<TypeId, HashMap<String, u32>>, Self::Error> {
        self.download_items(config_name).await
    }
}

#[async_trait]
impl ItemSetter for Client {
    type Error = Error;
    async fn set_items(
        &self,
        config_name: &str,
        items: HashMap<TypeId, HashMap<String, u32>>,
    ) -> Result<(), Self::Error> {
        self.upload_items(items, config_name).await
    }
}
