pub mod api;
pub mod request;
pub mod types;

pub use crate::internal::types::{
    AuthorizedAppType, AuthorizedAppsType, CreatedAuthorizedAppType, MailchimpErrorType, LinkType,
};

#[cfg(test)]
mod tests {
    use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
    use reqwest::Url;
    use serde_json::json;
    use std::collections::HashMap;

    use super::api::Api;
    use super::request::{BasicAuth, HttpReq, MailchimpResult};
    use super::types::{AuthorizedAppType, AuthorizedAppsType};

    ///
    ///
    /// Pruebas de respuestas para los diferentes ENDPOINTs
    ///
    struct MockRequest {
        resp_for_get: String,
        resp_for_post: String,
    }

    impl MockRequest {
        ///
        /// Inicializador de la clase
        ///
        pub fn new<'m>(resp_for_get: &'m str, resp_for_post: &'m str) -> Self {
            MockRequest {
                resp_for_get: resp_for_get.to_string(),
                resp_for_post: resp_for_post.to_string(),
            }
        }
    }

    impl HttpReq for MockRequest {
        ///
        ///  Argumentos:
        ///     url: Url
        ///     headers: HeaderMap
        ///
        fn get(
            &self,
            _url: Url,
            _headers: HeaderMap,
            _basic_auth: &Option<BasicAuth>,
        ) -> MailchimpResult<String> {
            Ok(self.resp_for_get.clone())
        }
        ///
        ///  Argumentos:
        ///     url: Url
        ///     headers: HeaderMap
        ///     payload: Datos a enviar a la URL especificada
        ///
        fn post(
            &self,
            _url: Url,
            _headers: HeaderMap,
            _payload: HashMap<String, String>,
            _basic_auth: &Option<BasicAuth>,
        ) -> MailchimpResult<String> {
            Ok(self.resp_for_post.clone())
        }
    }

    ///
    /// Configura la instancia de API para los diferentes Test que no requieren de
    /// una respuesta
    ///
    fn setup_test_with_access_token() -> Api<MockRequest> {
        Api::<MockRequest>::new("us6", "access_token", Box::new(MockRequest::new("", "")))
    }

    #[test]
    fn get_domain_url_with_access_token() {
        let api = setup_test_with_access_token();
        assert_eq!(api.domain(), "https://us6.api.mailchimp.com/");
    }
    #[test]
    fn get_api_version_with_access_token() {
        let api = setup_test_with_access_token();
        assert_eq!(api.api_version(), "3.0")
    }

    #[test]
    fn build_url_without_http_params() {
        let api = setup_test_with_access_token();
        assert_eq!(
            api.build_url("lists", &HashMap::new()).as_str(),
            "https://us6.api.mailchimp.com/3.0/lists"
        )
    }
    #[test]
    fn build_url_with_http_params() {
        let api = setup_test_with_access_token();
        let mut params = HashMap::new();
        params.insert("option1".to_string(), "foo".to_string());
        params.insert("option2".to_string(), "bar".to_string());
        assert_eq!(
            api.build_url("campaigns", &params).as_str(),
            "https://us6.api.mailchimp.com/3.0/campaigns?option1=foo&option2=bar"
        )
    }
    #[test]
    fn build_headers_contain_authorization_header() {
        let api = setup_test_with_access_token();
        let headers = api.build_headers();
        let h_auth_value = headers.get(AUTHORIZATION).unwrap();
        let h_ct_value = headers.get(CONTENT_TYPE).unwrap();

        assert_eq!(h_auth_value.to_str().unwrap(), "OAuth access_token");
        assert_eq!(h_ct_value.to_str().unwrap(), "application/json");
    }

    #[test]
    fn test_response_for_read_authorized_apps() {
        let mock_transport = MockRequest::new(
            "{ \"apps\": [{\"id\": 2486822,\"name\": \"Mailchimp for Shopify\",\"description\": \"Mailchimp for Shopify is a free application that connects your Shopify store with your Mailchimp account.\",\"users\": [ \"freddiesjokes\"],\"_links\": [ {\"rel\": \"self\",\"href\": \"https://usX.api.mailchimp.com/3.0/authorized-apps/2486822\",\"method\": \"GET\",\"targetSchema\": \"https://api.mailchimp.com/schema/3.0/AuthorizedAppType/Instance.json\" }, {\"rel\": \"parent\",\"href\": \"https://usX.api.mailchimp.com/3.0/authorized-apps\",\"method\": \"GET\",\"targetSchema\": \"https://api.mailchimp.com/schema/3.0/AuthorizedAppType/Collection.json\" }]},{\"id\": 26925,\"name\": \"Integrations Directory Reviews\",\"description\": \"Allow customers to leave feedback on the connect.mailchimp.com site.\",\"users\": [ \"freddiesjokes\"],\"_links\": [ {\"rel\": \"self\",\"href\": \"https://usX.api.mailchimp.com/3.0/authorized-apps/26925\",\"method\": \"GET\",\"targetSchema\": \"https://api.mailchimp.com/schema/3.0/AuthorizedAppType/Instance.json\" }, {\"rel\": \"parent\",\"href\": \"https://usX.api.mailchimp.com/3.0/authorized-apps\",\"method\": \"GET\",\"targetSchema\": \"https://api.mailchimp.com/schema/3.0/AuthorizedAppType/Collection.json\" }]},{\"id\": 27498,\"name\": \"Goooal\",\"description\": \"Segment your Mailchimp list based on subscribers' activity on your website.\",\"users\": [ \"freddiesjokes\"],\"_links\": [ {\"rel\": \"self\",\"href\": \"https://usX.api.mailchimp.com/3.0/authorized-apps/27498\",\"method\": \"GET\",\"targetSchema\": \"https://api.mailchimp.com/schema/3.0/AuthorizedAppType/Instance.json\" }, {\"rel\": \"parent\",\"href\": \"https://usX.api.mailchimp.com/3.0/authorized-apps\",\"method\": \"GET\",\"targetSchema\": \"https://api.mailchimp.com/schema/3.0/AuthorizedAppType/Collection.json\" }]} ], \"total_items\": 3, \"_links\": [{\"rel\": \"self\",\"href\": \"https://usX.api.mailchimp.com/3.0/authorized-apps\",\"method\": \"GET\",\"targetSchema\": \"https://api.mailchimp.com/schema/3.0/AuthorizedAppType/Collection.json\"},{\"rel\": \"parent\",\"href\": \"https://usX.api.mailchimp.com/3.0/\",\"method\": \"GET\",\"targetSchema\": \"https://api.mailchimp.com/schema/3.0/Root.json\"},{\"rel\": \"create\",\"href\": \"https://usX.api.mailchimp.com/3.0/authorized-apps\",\"method\": \"POST\",\"targetSchema\": \"https://api.mailchimp.com/schema/3.0/AuthorizedAppType/ClientAccessTokens.json\",\"schema\": \"https://api.mailchimp.com/schema/3.0/AuthorizedAppType/Client.json\"} ]}",
            "",
        );
        let api = Api::<MockRequest>::new("us6", "access_token", Box::new(mock_transport));

        let expected = json!({ "apps": [{"id": 2486822,"name": "Mailchimp for Shopify","description": "Mailchimp for Shopify is a free application that connects your Shopify store with your Mailchimp account.","users": [ "freddiesjokes"],"_links": [ {"rel": "self","href": "https://usX.api.mailchimp.com/3.0/authorized-apps/2486822","method": "GET","targetSchema": "https://api.mailchimp.com/schema/3.0/AuthorizedAppType/Instance.json" }, {"rel": "parent","href": "https://usX.api.mailchimp.com/3.0/authorized-apps","method": "GET","targetSchema": "https://api.mailchimp.com/schema/3.0/AuthorizedAppType/Collection.json" }]},{"id": 26925,"name": "Integrations Directory Reviews","description": "Allow customers to leave feedback on the connect.mailchimp.com site.","users": [ "freddiesjokes"],"_links": [ {"rel": "self","href": "https://usX.api.mailchimp.com/3.0/authorized-apps/26925","method": "GET","targetSchema": "https://api.mailchimp.com/schema/3.0/AuthorizedAppType/Instance.json" }, {"rel": "parent","href": "https://usX.api.mailchimp.com/3.0/authorized-apps","method": "GET","targetSchema": "https://api.mailchimp.com/schema/3.0/AuthorizedAppType/Collection.json" }]},{"id": 27498,"name": "Goooal","description": "Segment your Mailchimp list based on subscribers' activity on your website.","users": [ "freddiesjokes"],"_links": [ {"rel": "self","href": "https://usX.api.mailchimp.com/3.0/authorized-apps/27498","method": "GET","targetSchema": "https://api.mailchimp.com/schema/3.0/AuthorizedAppType/Instance.json" }, {"rel": "parent","href": "https://usX.api.mailchimp.com/3.0/authorized-apps","method": "GET","targetSchema": "https://api.mailchimp.com/schema/3.0/AuthorizedAppType/Collection.json" }]} ], "total_items": 3, "_links": [{"rel": "self","href": "https://usX.api.mailchimp.com/3.0/authorized-apps","method": "GET","targetSchema": "https://api.mailchimp.com/schema/3.0/AuthorizedAppType/Collection.json"},{"rel": "parent","href": "https://usX.api.mailchimp.com/3.0/","method": "GET","targetSchema": "https://api.mailchimp.com/schema/3.0/Root.json"},{"rel": "create","href": "https://usX.api.mailchimp.com/3.0/authorized-apps","method": "POST","targetSchema": "https://api.mailchimp.com/schema/3.0/AuthorizedAppType/ClientAccessTokens.json","schema": "https://api.mailchimp.com/schema/3.0/AuthorizedAppType/Client.json"} ]});

        let resp = api
            .get_edge::<AuthorizedAppsType>("authorized-apps", HashMap::new())
            .unwrap();

        assert_eq!(
            resp.apps.len(), expected["total_items"],
            "Los estados de la petición no coinciden: Valor de la respuesta {:?} Valor esperado: {:?}",
            resp, expected
        );

        for it in 1..resp.apps.len() {
            assert_eq!(
                resp.apps[it].id, expected["apps"][it]["id"],
                "Los valores no coinciden: Valor respondido {:?} Valor esperado: {:?}",
                resp, expected
            );
        }
    }
}
