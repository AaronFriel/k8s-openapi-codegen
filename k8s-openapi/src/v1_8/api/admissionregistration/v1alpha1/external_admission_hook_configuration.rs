// Generated from definition io.k8s.api.admissionregistration.v1alpha1.ExternalAdmissionHookConfiguration

/// ExternalAdmissionHookConfiguration describes the configuration of initializers.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ExternalAdmissionHookConfiguration {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    pub api_version: Option<String>,

    /// ExternalAdmissionHooks is a list of external admission webhooks and the affected resources and operations.
    pub external_admission_hooks: Option<Vec<::v1_8::api::admissionregistration::v1alpha1::ExternalAdmissionHook>>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    pub kind: Option<String>,

    /// Standard object metadata; More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata.
    pub metadata: Option<::v1_8::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
}

// Begin admissionregistration.k8s.io/v1alpha1/ExternalAdmissionHookConfiguration

// Generated from operation createAdmissionregistrationV1alpha1ExternalAdmissionHookConfiguration

impl ExternalAdmissionHookConfiguration {
    /// create an ExternalAdmissionHookConfiguration
    ///
    /// Use [`CreateAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationResponse`](./enum.CreateAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `body`
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn create_admissionregistration_v1alpha1_external_admission_hook_configuration(
        body: &::v1_8::api::admissionregistration::v1alpha1::ExternalAdmissionHookConfiguration,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/admissionregistration.k8s.io/v1alpha1/externaladmissionhookconfigurations?");
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::post(__url);
        let __body = ::serde_json::to_vec(&body).map_err(::RequestError::Json)?;
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`ExternalAdmissionHookConfiguration::create_admissionregistration_v1alpha1_external_admission_hook_configuration`](./struct.ExternalAdmissionHookConfiguration.html#method.create_admissionregistration_v1alpha1_external_admission_hook_configuration)
#[derive(Debug)]
pub enum CreateAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationResponse {
    Ok(::v1_8::api::admissionregistration::v1alpha1::ExternalAdmissionHookConfiguration),
    Unauthorized,
    Other,
}

impl ::Response for CreateAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((CreateAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((CreateAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationResponse::Unauthorized, 0)),
            _ => Ok((CreateAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteAdmissionregistrationV1alpha1CollectionExternalAdmissionHookConfiguration

impl ExternalAdmissionHookConfiguration {
    /// delete collection of ExternalAdmissionHookConfiguration
    ///
    /// Use [`DeleteAdmissionregistrationV1alpha1CollectionExternalAdmissionHookConfigurationResponse`](./enum.DeleteAdmissionregistrationV1alpha1CollectionExternalAdmissionHookConfigurationResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `continue_`
    ///
    ///     The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
    ///
    /// * `field_selector`
    ///
    ///     A selector to restrict the list of returned objects by their fields. Defaults to everything.
    ///
    /// * `include_uninitialized`
    ///
    ///     If true, partially initialized resources are included in the response.
    ///
    /// * `label_selector`
    ///
    ///     A selector to restrict the list of returned objects by their labels. Defaults to everything.
    ///
    /// * `limit`
    ///
    ///     limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
    ///
    ///     The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    ///
    /// * `resource_version`
    ///
    ///     When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    ///
    /// * `timeout_seconds`
    ///
    ///     Timeout for the list/watch call.
    ///
    /// * `watch`
    ///
    ///     Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub fn delete_admissionregistration_v1alpha1_collection_external_admission_hook_configuration(
        continue_: Option<&str>,
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        limit: Option<i64>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/admissionregistration.k8s.io/v1alpha1/externaladmissionhookconfigurations?");
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(continue_) = continue_ {
            __query_pairs.append_pair("continue", continue_);
        }
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
        }
        if let Some(limit) = limit {
            __query_pairs.append_pair("limit", &limit.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(resource_version) = resource_version {
            __query_pairs.append_pair("resourceVersion", resource_version);
        }
        if let Some(timeout_seconds) = timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
        }
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::delete(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`ExternalAdmissionHookConfiguration::delete_admissionregistration_v1alpha1_collection_external_admission_hook_configuration`](./struct.ExternalAdmissionHookConfiguration.html#method.delete_admissionregistration_v1alpha1_collection_external_admission_hook_configuration)
#[derive(Debug)]
pub enum DeleteAdmissionregistrationV1alpha1CollectionExternalAdmissionHookConfigurationResponse {
    OkStatus(::v1_8::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(::v1_8::api::admissionregistration::v1alpha1::ExternalAdmissionHookConfiguration),
    Unauthorized,
    Other,
}

impl ::Response for DeleteAdmissionregistrationV1alpha1CollectionExternalAdmissionHookConfigurationResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result: ::serde_json::Map<String, ::serde_json::Value> = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                let is_status = match result.get("kind") {
                    Some(::serde_json::Value::String(s)) if s == "Status" => true,
                    _ => false,
                };
                if is_status {
                    let result = ::serde::Deserialize::deserialize(::serde_json::Value::Object(result));
                    let result = result.map_err(::ResponseError::Json)?;
                    Ok((DeleteAdmissionregistrationV1alpha1CollectionExternalAdmissionHookConfigurationResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = ::serde::Deserialize::deserialize(::serde_json::Value::Object(result));
                    let result = result.map_err(::ResponseError::Json)?;
                    Ok((DeleteAdmissionregistrationV1alpha1CollectionExternalAdmissionHookConfigurationResponse::OkValue(result), buf.len()))
                }
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((DeleteAdmissionregistrationV1alpha1CollectionExternalAdmissionHookConfigurationResponse::Unauthorized, 0)),
            _ => Ok((DeleteAdmissionregistrationV1alpha1CollectionExternalAdmissionHookConfigurationResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteAdmissionregistrationV1alpha1ExternalAdmissionHookConfiguration

impl ExternalAdmissionHookConfiguration {
    /// delete an ExternalAdmissionHookConfiguration
    ///
    /// Use [`DeleteAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationResponse`](./enum.DeleteAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ExternalAdmissionHookConfiguration
    ///
    /// * `body`
    ///
    /// * `grace_period_seconds`
    ///
    ///     The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately.
    ///
    /// * `orphan_dependents`
    ///
    ///     Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the "orphan" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both.
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    ///
    /// * `propagation_policy`
    ///
    ///     Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy.
    pub fn delete_admissionregistration_v1alpha1_external_admission_hook_configuration(
        name: &str,
        grace_period_seconds: Option<i64>,
        orphan_dependents: Option<bool>,
        pretty: Option<&str>,
        propagation_policy: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/admissionregistration.k8s.io/v1alpha1/externaladmissionhookconfigurations/{name}?", name = name);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(grace_period_seconds) = grace_period_seconds {
            __query_pairs.append_pair("gracePeriodSeconds", &grace_period_seconds.to_string());
        }
        if let Some(orphan_dependents) = orphan_dependents {
            __query_pairs.append_pair("orphanDependents", &orphan_dependents.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(propagation_policy) = propagation_policy {
            __query_pairs.append_pair("propagationPolicy", propagation_policy);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::delete(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`ExternalAdmissionHookConfiguration::delete_admissionregistration_v1alpha1_external_admission_hook_configuration`](./struct.ExternalAdmissionHookConfiguration.html#method.delete_admissionregistration_v1alpha1_external_admission_hook_configuration)
#[derive(Debug)]
pub enum DeleteAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationResponse {
    OkStatus(::v1_8::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(::v1_8::api::admissionregistration::v1alpha1::ExternalAdmissionHookConfiguration),
    Unauthorized,
    Other,
}

impl ::Response for DeleteAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result: ::serde_json::Map<String, ::serde_json::Value> = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                let is_status = match result.get("kind") {
                    Some(::serde_json::Value::String(s)) if s == "Status" => true,
                    _ => false,
                };
                if is_status {
                    let result = ::serde::Deserialize::deserialize(::serde_json::Value::Object(result));
                    let result = result.map_err(::ResponseError::Json)?;
                    Ok((DeleteAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = ::serde::Deserialize::deserialize(::serde_json::Value::Object(result));
                    let result = result.map_err(::ResponseError::Json)?;
                    Ok((DeleteAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationResponse::OkValue(result), buf.len()))
                }
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((DeleteAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationResponse::Unauthorized, 0)),
            _ => Ok((DeleteAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationResponse::Other, 0)),
        }
    }
}

// Generated from operation listAdmissionregistrationV1alpha1ExternalAdmissionHookConfiguration

impl ExternalAdmissionHookConfiguration {
    /// list or watch objects of kind ExternalAdmissionHookConfiguration
    ///
    /// Use [`ListAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationResponse`](./enum.ListAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `continue_`
    ///
    ///     The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
    ///
    /// * `field_selector`
    ///
    ///     A selector to restrict the list of returned objects by their fields. Defaults to everything.
    ///
    /// * `include_uninitialized`
    ///
    ///     If true, partially initialized resources are included in the response.
    ///
    /// * `label_selector`
    ///
    ///     A selector to restrict the list of returned objects by their labels. Defaults to everything.
    ///
    /// * `limit`
    ///
    ///     limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
    ///
    ///     The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    ///
    /// * `resource_version`
    ///
    ///     When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    ///
    /// * `timeout_seconds`
    ///
    ///     Timeout for the list/watch call.
    ///
    /// * `watch`
    ///
    ///     Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub fn list_admissionregistration_v1alpha1_external_admission_hook_configuration(
        continue_: Option<&str>,
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        limit: Option<i64>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/admissionregistration.k8s.io/v1alpha1/externaladmissionhookconfigurations?");
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(continue_) = continue_ {
            __query_pairs.append_pair("continue", continue_);
        }
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
        }
        if let Some(limit) = limit {
            __query_pairs.append_pair("limit", &limit.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(resource_version) = resource_version {
            __query_pairs.append_pair("resourceVersion", resource_version);
        }
        if let Some(timeout_seconds) = timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
        }
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`ExternalAdmissionHookConfiguration::list_admissionregistration_v1alpha1_external_admission_hook_configuration`](./struct.ExternalAdmissionHookConfiguration.html#method.list_admissionregistration_v1alpha1_external_admission_hook_configuration)
#[derive(Debug)]
pub enum ListAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationResponse {
    Ok(::v1_8::api::admissionregistration::v1alpha1::ExternalAdmissionHookConfigurationList),
    Unauthorized,
    Other,
}

impl ::Response for ListAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ListAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ListAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationResponse::Unauthorized, 0)),
            _ => Ok((ListAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationResponse::Other, 0)),
        }
    }
}

// Generated from operation patchAdmissionregistrationV1alpha1ExternalAdmissionHookConfiguration

impl ExternalAdmissionHookConfiguration {
    /// partially update the specified ExternalAdmissionHookConfiguration
    ///
    /// Use [`PatchAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationResponse`](./enum.PatchAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ExternalAdmissionHookConfiguration
    ///
    /// * `body`
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn patch_admissionregistration_v1alpha1_external_admission_hook_configuration(
        name: &str,
        body: &::v1_8::apimachinery::pkg::apis::meta::v1::Patch,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/admissionregistration.k8s.io/v1alpha1/externaladmissionhookconfigurations/{name}?", name = name);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::patch(__url);
        let __body = ::serde_json::to_vec(&body).map_err(::RequestError::Json)?;
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`ExternalAdmissionHookConfiguration::patch_admissionregistration_v1alpha1_external_admission_hook_configuration`](./struct.ExternalAdmissionHookConfiguration.html#method.patch_admissionregistration_v1alpha1_external_admission_hook_configuration)
#[derive(Debug)]
pub enum PatchAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationResponse {
    Ok(::v1_8::api::admissionregistration::v1alpha1::ExternalAdmissionHookConfiguration),
    Unauthorized,
    Other,
}

impl ::Response for PatchAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((PatchAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((PatchAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationResponse::Unauthorized, 0)),
            _ => Ok((PatchAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationResponse::Other, 0)),
        }
    }
}

// Generated from operation readAdmissionregistrationV1alpha1ExternalAdmissionHookConfiguration

impl ExternalAdmissionHookConfiguration {
    /// read the specified ExternalAdmissionHookConfiguration
    ///
    /// Use [`ReadAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationResponse`](./enum.ReadAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ExternalAdmissionHookConfiguration
    ///
    /// * `exact`
    ///
    ///     Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
    ///
    /// * `export`
    ///
    ///     Should this value be exported.  Export strips fields that a user can not specify.
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn read_admissionregistration_v1alpha1_external_admission_hook_configuration(
        name: &str,
        exact: Option<bool>,
        export: Option<bool>,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/admissionregistration.k8s.io/v1alpha1/externaladmissionhookconfigurations/{name}?", name = name);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(exact) = exact {
            __query_pairs.append_pair("exact", &exact.to_string());
        }
        if let Some(export) = export {
            __query_pairs.append_pair("export", &export.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`ExternalAdmissionHookConfiguration::read_admissionregistration_v1alpha1_external_admission_hook_configuration`](./struct.ExternalAdmissionHookConfiguration.html#method.read_admissionregistration_v1alpha1_external_admission_hook_configuration)
#[derive(Debug)]
pub enum ReadAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationResponse {
    Ok(::v1_8::api::admissionregistration::v1alpha1::ExternalAdmissionHookConfiguration),
    Unauthorized,
    Other,
}

impl ::Response for ReadAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReadAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ReadAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationResponse::Unauthorized, 0)),
            _ => Ok((ReadAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceAdmissionregistrationV1alpha1ExternalAdmissionHookConfiguration

impl ExternalAdmissionHookConfiguration {
    /// replace the specified ExternalAdmissionHookConfiguration
    ///
    /// Use [`ReplaceAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationResponse`](./enum.ReplaceAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ExternalAdmissionHookConfiguration
    ///
    /// * `body`
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn replace_admissionregistration_v1alpha1_external_admission_hook_configuration(
        name: &str,
        body: &::v1_8::api::admissionregistration::v1alpha1::ExternalAdmissionHookConfiguration,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/admissionregistration.k8s.io/v1alpha1/externaladmissionhookconfigurations/{name}?", name = name);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::put(__url);
        let __body = ::serde_json::to_vec(&body).map_err(::RequestError::Json)?;
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`ExternalAdmissionHookConfiguration::replace_admissionregistration_v1alpha1_external_admission_hook_configuration`](./struct.ExternalAdmissionHookConfiguration.html#method.replace_admissionregistration_v1alpha1_external_admission_hook_configuration)
#[derive(Debug)]
pub enum ReplaceAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationResponse {
    Ok(::v1_8::api::admissionregistration::v1alpha1::ExternalAdmissionHookConfiguration),
    Unauthorized,
    Other,
}

impl ::Response for ReplaceAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReplaceAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ReplaceAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationResponse::Unauthorized, 0)),
            _ => Ok((ReplaceAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationResponse::Other, 0)),
        }
    }
}

// Generated from operation watchAdmissionregistrationV1alpha1ExternalAdmissionHookConfiguration

impl ExternalAdmissionHookConfiguration {
    /// watch changes to an object of kind ExternalAdmissionHookConfiguration
    ///
    /// Use [`WatchAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationResponse`](./enum.WatchAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the ExternalAdmissionHookConfiguration
    ///
    /// * `continue_`
    ///
    ///     The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
    ///
    /// * `field_selector`
    ///
    ///     A selector to restrict the list of returned objects by their fields. Defaults to everything.
    ///
    /// * `include_uninitialized`
    ///
    ///     If true, partially initialized resources are included in the response.
    ///
    /// * `label_selector`
    ///
    ///     A selector to restrict the list of returned objects by their labels. Defaults to everything.
    ///
    /// * `limit`
    ///
    ///     limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
    ///
    ///     The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    ///
    /// * `resource_version`
    ///
    ///     When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    ///
    /// * `timeout_seconds`
    ///
    ///     Timeout for the list/watch call.
    ///
    /// * `watch`
    ///
    ///     Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub fn watch_admissionregistration_v1alpha1_external_admission_hook_configuration(
        name: &str,
        continue_: Option<&str>,
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        limit: Option<i64>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/admissionregistration.k8s.io/v1alpha1/watch/externaladmissionhookconfigurations/{name}?", name = name);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(continue_) = continue_ {
            __query_pairs.append_pair("continue", continue_);
        }
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
        }
        if let Some(limit) = limit {
            __query_pairs.append_pair("limit", &limit.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(resource_version) = resource_version {
            __query_pairs.append_pair("resourceVersion", resource_version);
        }
        if let Some(timeout_seconds) = timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
        }
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`ExternalAdmissionHookConfiguration::watch_admissionregistration_v1alpha1_external_admission_hook_configuration`](./struct.ExternalAdmissionHookConfiguration.html#method.watch_admissionregistration_v1alpha1_external_admission_hook_configuration)
#[derive(Debug)]
pub enum WatchAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationResponse {
    Ok(::v1_8::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl ::Response for WatchAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let mut deserializer = ::serde_json::Deserializer::from_slice(buf).into_iter();
                let (result, byte_offset) = match deserializer.next() {
                    Some(Ok(value)) => (value, deserializer.byte_offset()),
                    Some(Err(ref err)) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Some(Err(err)) => return Err(::ResponseError::Json(err)),
                    None => return Err(::ResponseError::NeedMoreData),
                };
                Ok((WatchAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationResponse::Ok(result), byte_offset))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((WatchAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationResponse::Unauthorized, 0)),
            _ => Ok((WatchAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationResponse::Other, 0)),
        }
    }
}

// Generated from operation watchAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationList

impl ExternalAdmissionHookConfiguration {
    /// watch individual changes to a list of ExternalAdmissionHookConfiguration
    ///
    /// Use [`WatchAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationListResponse`](./enum.WatchAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationListResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `continue_`
    ///
    ///     The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
    ///
    /// * `field_selector`
    ///
    ///     A selector to restrict the list of returned objects by their fields. Defaults to everything.
    ///
    /// * `include_uninitialized`
    ///
    ///     If true, partially initialized resources are included in the response.
    ///
    /// * `label_selector`
    ///
    ///     A selector to restrict the list of returned objects by their labels. Defaults to everything.
    ///
    /// * `limit`
    ///
    ///     limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
    ///
    ///     The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    ///
    /// * `resource_version`
    ///
    ///     When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
    ///
    /// * `timeout_seconds`
    ///
    ///     Timeout for the list/watch call.
    ///
    /// * `watch`
    ///
    ///     Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
    pub fn watch_admissionregistration_v1alpha1_external_admission_hook_configuration_list(
        continue_: Option<&str>,
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        limit: Option<i64>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/admissionregistration.k8s.io/v1alpha1/watch/externaladmissionhookconfigurations?");
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(continue_) = continue_ {
            __query_pairs.append_pair("continue", continue_);
        }
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
        }
        if let Some(limit) = limit {
            __query_pairs.append_pair("limit", &limit.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", pretty);
        }
        if let Some(resource_version) = resource_version {
            __query_pairs.append_pair("resourceVersion", resource_version);
        }
        if let Some(timeout_seconds) = timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
        }
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

/// Parses the HTTP response of [`ExternalAdmissionHookConfiguration::watch_admissionregistration_v1alpha1_external_admission_hook_configuration_list`](./struct.ExternalAdmissionHookConfiguration.html#method.watch_admissionregistration_v1alpha1_external_admission_hook_configuration_list)
#[derive(Debug)]
pub enum WatchAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationListResponse {
    Ok(::v1_8::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl ::Response for WatchAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationListResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let mut deserializer = ::serde_json::Deserializer::from_slice(buf).into_iter();
                let (result, byte_offset) = match deserializer.next() {
                    Some(Ok(value)) => (value, deserializer.byte_offset()),
                    Some(Err(ref err)) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Some(Err(err)) => return Err(::ResponseError::Json(err)),
                    None => return Err(::ResponseError::NeedMoreData),
                };
                Ok((WatchAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationListResponse::Ok(result), byte_offset))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((WatchAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationListResponse::Unauthorized, 0)),
            _ => Ok((WatchAdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationListResponse::Other, 0)),
        }
    }
}

// End admissionregistration.k8s.io/v1alpha1/ExternalAdmissionHookConfiguration

impl<'de> ::serde::Deserialize<'de> for ExternalAdmissionHookConfiguration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_external_admission_hooks,
            Key_kind,
            Key_metadata,
            Other,
        }

        impl<'de> ::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        write!(f, "field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: ::serde::de::Error {
                        Ok(match v {
                            "apiVersion" => Field::Key_api_version,
                            "externalAdmissionHooks" => Field::Key_external_admission_hooks,
                            "kind" => Field::Key_kind,
                            "metadata" => Field::Key_metadata,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ExternalAdmissionHookConfiguration;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct ExternalAdmissionHookConfiguration")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_api_version: Option<String> = None;
                let mut value_external_admission_hooks: Option<Vec<::v1_8::api::admissionregistration::v1alpha1::ExternalAdmissionHook>> = None;
                let mut value_kind: Option<String> = None;
                let mut value_metadata: Option<::v1_8::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => value_api_version = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_external_admission_hooks => value_external_admission_hooks = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metadata => value_metadata = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ExternalAdmissionHookConfiguration {
                    api_version: value_api_version,
                    external_admission_hooks: value_external_admission_hooks,
                    kind: value_kind,
                    metadata: value_metadata,
                })
            }
        }

        deserializer.deserialize_struct(
            "ExternalAdmissionHookConfiguration",
            &[
                "apiVersion",
                "externalAdmissionHooks",
                "kind",
                "metadata",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for ExternalAdmissionHookConfiguration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ExternalAdmissionHookConfiguration",
            0 +
            self.api_version.as_ref().map_or(0, |_| 1) +
            self.external_admission_hooks.as_ref().map_or(0, |_| 1) +
            self.kind.as_ref().map_or(0, |_| 1) +
            self.metadata.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.api_version {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", value)?;
        }
        if let Some(value) = &self.external_admission_hooks {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "externalAdmissionHooks", value)?;
        }
        if let Some(value) = &self.kind {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", value)?;
        }
        if let Some(value) = &self.metadata {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
