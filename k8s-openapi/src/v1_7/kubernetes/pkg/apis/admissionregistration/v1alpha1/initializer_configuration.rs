// Generated from definition io.k8s.kubernetes.pkg.apis.admissionregistration.v1alpha1.InitializerConfiguration

/// InitializerConfiguration describes the configuration of initializers.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct InitializerConfiguration {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    pub api_version: Option<String>,

    /// Initializers is a list of resources and their default initializers Order-sensitive. When merging multiple InitializerConfigurations, we sort the initializers from different InitializerConfigurations by the name of the InitializerConfigurations; the order of the initializers from the same InitializerConfiguration is preserved.
    pub initializers: Option<Vec<::v1_7::kubernetes::pkg::apis::admissionregistration::v1alpha1::Initializer>>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    pub kind: Option<String>,

    /// Standard object metadata; More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata.
    pub metadata: Option<::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
}

// Begin admissionregistration.k8s.io/v1alpha1/InitializerConfiguration

// Generated from operation createAdmissionregistrationV1alpha1InitializerConfiguration

impl InitializerConfiguration {
    /// create an InitializerConfiguration
    ///
    /// Use [`CreateAdmissionregistrationV1alpha1InitializerConfigurationResponse`](./enum.CreateAdmissionregistrationV1alpha1InitializerConfigurationResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `body`
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn create_admissionregistration_v1alpha1_initializer_configuration(
        body: &::v1_7::kubernetes::pkg::apis::admissionregistration::v1alpha1::InitializerConfiguration,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/admissionregistration.k8s.io/v1alpha1/initializerconfigurations?");
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

/// Parses the HTTP response of [`InitializerConfiguration::create_admissionregistration_v1alpha1_initializer_configuration`](./struct.InitializerConfiguration.html#method.create_admissionregistration_v1alpha1_initializer_configuration)
#[derive(Debug)]
pub enum CreateAdmissionregistrationV1alpha1InitializerConfigurationResponse {
    Ok(::v1_7::kubernetes::pkg::apis::admissionregistration::v1alpha1::InitializerConfiguration),
    Unauthorized,
    Other,
}

impl ::Response for CreateAdmissionregistrationV1alpha1InitializerConfigurationResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((CreateAdmissionregistrationV1alpha1InitializerConfigurationResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((CreateAdmissionregistrationV1alpha1InitializerConfigurationResponse::Unauthorized, 0)),
            _ => Ok((CreateAdmissionregistrationV1alpha1InitializerConfigurationResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteAdmissionregistrationV1alpha1CollectionInitializerConfiguration

impl InitializerConfiguration {
    /// delete collection of InitializerConfiguration
    ///
    /// Use [`DeleteAdmissionregistrationV1alpha1CollectionInitializerConfigurationResponse`](./enum.DeleteAdmissionregistrationV1alpha1CollectionInitializerConfigurationResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
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
    pub fn delete_admissionregistration_v1alpha1_collection_initializer_configuration(
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/admissionregistration.k8s.io/v1alpha1/initializerconfigurations?");
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
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

/// Parses the HTTP response of [`InitializerConfiguration::delete_admissionregistration_v1alpha1_collection_initializer_configuration`](./struct.InitializerConfiguration.html#method.delete_admissionregistration_v1alpha1_collection_initializer_configuration)
#[derive(Debug)]
pub enum DeleteAdmissionregistrationV1alpha1CollectionInitializerConfigurationResponse {
    OkStatus(::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(::v1_7::kubernetes::pkg::apis::admissionregistration::v1alpha1::InitializerConfiguration),
    Unauthorized,
    Other,
}

impl ::Response for DeleteAdmissionregistrationV1alpha1CollectionInitializerConfigurationResponse {
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
                    Ok((DeleteAdmissionregistrationV1alpha1CollectionInitializerConfigurationResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = ::serde::Deserialize::deserialize(::serde_json::Value::Object(result));
                    let result = result.map_err(::ResponseError::Json)?;
                    Ok((DeleteAdmissionregistrationV1alpha1CollectionInitializerConfigurationResponse::OkValue(result), buf.len()))
                }
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((DeleteAdmissionregistrationV1alpha1CollectionInitializerConfigurationResponse::Unauthorized, 0)),
            _ => Ok((DeleteAdmissionregistrationV1alpha1CollectionInitializerConfigurationResponse::Other, 0)),
        }
    }
}

// Generated from operation deleteAdmissionregistrationV1alpha1InitializerConfiguration

impl InitializerConfiguration {
    /// delete an InitializerConfiguration
    ///
    /// Use [`DeleteAdmissionregistrationV1alpha1InitializerConfigurationResponse`](./enum.DeleteAdmissionregistrationV1alpha1InitializerConfigurationResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the InitializerConfiguration
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
    pub fn delete_admissionregistration_v1alpha1_initializer_configuration(
        name: &str,
        grace_period_seconds: Option<i64>,
        orphan_dependents: Option<bool>,
        pretty: Option<&str>,
        propagation_policy: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/admissionregistration.k8s.io/v1alpha1/initializerconfigurations/{name}?", name = name);
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

/// Parses the HTTP response of [`InitializerConfiguration::delete_admissionregistration_v1alpha1_initializer_configuration`](./struct.InitializerConfiguration.html#method.delete_admissionregistration_v1alpha1_initializer_configuration)
#[derive(Debug)]
pub enum DeleteAdmissionregistrationV1alpha1InitializerConfigurationResponse {
    OkStatus(::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    OkValue(::v1_7::kubernetes::pkg::apis::admissionregistration::v1alpha1::InitializerConfiguration),
    Unauthorized,
    Other,
}

impl ::Response for DeleteAdmissionregistrationV1alpha1InitializerConfigurationResponse {
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
                    Ok((DeleteAdmissionregistrationV1alpha1InitializerConfigurationResponse::OkStatus(result), buf.len()))
                }
                else {
                    let result = ::serde::Deserialize::deserialize(::serde_json::Value::Object(result));
                    let result = result.map_err(::ResponseError::Json)?;
                    Ok((DeleteAdmissionregistrationV1alpha1InitializerConfigurationResponse::OkValue(result), buf.len()))
                }
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((DeleteAdmissionregistrationV1alpha1InitializerConfigurationResponse::Unauthorized, 0)),
            _ => Ok((DeleteAdmissionregistrationV1alpha1InitializerConfigurationResponse::Other, 0)),
        }
    }
}

// Generated from operation listAdmissionregistrationV1alpha1InitializerConfiguration

impl InitializerConfiguration {
    /// list or watch objects of kind InitializerConfiguration
    ///
    /// Use [`ListAdmissionregistrationV1alpha1InitializerConfigurationResponse`](./enum.ListAdmissionregistrationV1alpha1InitializerConfigurationResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
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
    pub fn list_admissionregistration_v1alpha1_initializer_configuration(
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/admissionregistration.k8s.io/v1alpha1/initializerconfigurations?");
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
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

/// Parses the HTTP response of [`InitializerConfiguration::list_admissionregistration_v1alpha1_initializer_configuration`](./struct.InitializerConfiguration.html#method.list_admissionregistration_v1alpha1_initializer_configuration)
#[derive(Debug)]
pub enum ListAdmissionregistrationV1alpha1InitializerConfigurationResponse {
    Ok(::v1_7::kubernetes::pkg::apis::admissionregistration::v1alpha1::InitializerConfigurationList),
    Unauthorized,
    Other,
}

impl ::Response for ListAdmissionregistrationV1alpha1InitializerConfigurationResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ListAdmissionregistrationV1alpha1InitializerConfigurationResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ListAdmissionregistrationV1alpha1InitializerConfigurationResponse::Unauthorized, 0)),
            _ => Ok((ListAdmissionregistrationV1alpha1InitializerConfigurationResponse::Other, 0)),
        }
    }
}

// Generated from operation patchAdmissionregistrationV1alpha1InitializerConfiguration

impl InitializerConfiguration {
    /// partially update the specified InitializerConfiguration
    ///
    /// Use [`PatchAdmissionregistrationV1alpha1InitializerConfigurationResponse`](./enum.PatchAdmissionregistrationV1alpha1InitializerConfigurationResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the InitializerConfiguration
    ///
    /// * `body`
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn patch_admissionregistration_v1alpha1_initializer_configuration(
        name: &str,
        body: &::v1_7::apimachinery::pkg::apis::meta::v1::Patch,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/admissionregistration.k8s.io/v1alpha1/initializerconfigurations/{name}?", name = name);
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

/// Parses the HTTP response of [`InitializerConfiguration::patch_admissionregistration_v1alpha1_initializer_configuration`](./struct.InitializerConfiguration.html#method.patch_admissionregistration_v1alpha1_initializer_configuration)
#[derive(Debug)]
pub enum PatchAdmissionregistrationV1alpha1InitializerConfigurationResponse {
    Ok(::v1_7::kubernetes::pkg::apis::admissionregistration::v1alpha1::InitializerConfiguration),
    Unauthorized,
    Other,
}

impl ::Response for PatchAdmissionregistrationV1alpha1InitializerConfigurationResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((PatchAdmissionregistrationV1alpha1InitializerConfigurationResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((PatchAdmissionregistrationV1alpha1InitializerConfigurationResponse::Unauthorized, 0)),
            _ => Ok((PatchAdmissionregistrationV1alpha1InitializerConfigurationResponse::Other, 0)),
        }
    }
}

// Generated from operation readAdmissionregistrationV1alpha1InitializerConfiguration

impl InitializerConfiguration {
    /// read the specified InitializerConfiguration
    ///
    /// Use [`ReadAdmissionregistrationV1alpha1InitializerConfigurationResponse`](./enum.ReadAdmissionregistrationV1alpha1InitializerConfigurationResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the InitializerConfiguration
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
    pub fn read_admissionregistration_v1alpha1_initializer_configuration(
        name: &str,
        exact: Option<bool>,
        export: Option<bool>,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/admissionregistration.k8s.io/v1alpha1/initializerconfigurations/{name}?", name = name);
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

/// Parses the HTTP response of [`InitializerConfiguration::read_admissionregistration_v1alpha1_initializer_configuration`](./struct.InitializerConfiguration.html#method.read_admissionregistration_v1alpha1_initializer_configuration)
#[derive(Debug)]
pub enum ReadAdmissionregistrationV1alpha1InitializerConfigurationResponse {
    Ok(::v1_7::kubernetes::pkg::apis::admissionregistration::v1alpha1::InitializerConfiguration),
    Unauthorized,
    Other,
}

impl ::Response for ReadAdmissionregistrationV1alpha1InitializerConfigurationResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReadAdmissionregistrationV1alpha1InitializerConfigurationResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ReadAdmissionregistrationV1alpha1InitializerConfigurationResponse::Unauthorized, 0)),
            _ => Ok((ReadAdmissionregistrationV1alpha1InitializerConfigurationResponse::Other, 0)),
        }
    }
}

// Generated from operation replaceAdmissionregistrationV1alpha1InitializerConfiguration

impl InitializerConfiguration {
    /// replace the specified InitializerConfiguration
    ///
    /// Use [`ReplaceAdmissionregistrationV1alpha1InitializerConfigurationResponse`](./enum.ReplaceAdmissionregistrationV1alpha1InitializerConfigurationResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the InitializerConfiguration
    ///
    /// * `body`
    ///
    /// * `pretty`
    ///
    ///     If 'true', then the output is pretty printed.
    pub fn replace_admissionregistration_v1alpha1_initializer_configuration(
        name: &str,
        body: &::v1_7::kubernetes::pkg::apis::admissionregistration::v1alpha1::InitializerConfiguration,
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/admissionregistration.k8s.io/v1alpha1/initializerconfigurations/{name}?", name = name);
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

/// Parses the HTTP response of [`InitializerConfiguration::replace_admissionregistration_v1alpha1_initializer_configuration`](./struct.InitializerConfiguration.html#method.replace_admissionregistration_v1alpha1_initializer_configuration)
#[derive(Debug)]
pub enum ReplaceAdmissionregistrationV1alpha1InitializerConfigurationResponse {
    Ok(::v1_7::kubernetes::pkg::apis::admissionregistration::v1alpha1::InitializerConfiguration),
    Unauthorized,
    Other,
}

impl ::Response for ReplaceAdmissionregistrationV1alpha1InitializerConfigurationResponse {
    fn try_from_parts(status_code: ::http::StatusCode, buf: &[u8]) -> Result<(Self, usize), ::ResponseError> {
        match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                Ok((ReplaceAdmissionregistrationV1alpha1InitializerConfigurationResponse::Ok(result), buf.len()))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((ReplaceAdmissionregistrationV1alpha1InitializerConfigurationResponse::Unauthorized, 0)),
            _ => Ok((ReplaceAdmissionregistrationV1alpha1InitializerConfigurationResponse::Other, 0)),
        }
    }
}

// Generated from operation watchAdmissionregistrationV1alpha1InitializerConfiguration

impl InitializerConfiguration {
    /// watch changes to an object of kind InitializerConfiguration
    ///
    /// Use [`WatchAdmissionregistrationV1alpha1InitializerConfigurationResponse`](./enum.WatchAdmissionregistrationV1alpha1InitializerConfigurationResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the InitializerConfiguration
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
    pub fn watch_admissionregistration_v1alpha1_initializer_configuration(
        name: &str,
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/admissionregistration.k8s.io/v1alpha1/watch/initializerconfigurations/{name}?", name = name);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
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

/// Parses the HTTP response of [`InitializerConfiguration::watch_admissionregistration_v1alpha1_initializer_configuration`](./struct.InitializerConfiguration.html#method.watch_admissionregistration_v1alpha1_initializer_configuration)
#[derive(Debug)]
pub enum WatchAdmissionregistrationV1alpha1InitializerConfigurationResponse {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl ::Response for WatchAdmissionregistrationV1alpha1InitializerConfigurationResponse {
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
                Ok((WatchAdmissionregistrationV1alpha1InitializerConfigurationResponse::Ok(result), byte_offset))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((WatchAdmissionregistrationV1alpha1InitializerConfigurationResponse::Unauthorized, 0)),
            _ => Ok((WatchAdmissionregistrationV1alpha1InitializerConfigurationResponse::Other, 0)),
        }
    }
}

// Generated from operation watchAdmissionregistrationV1alpha1InitializerConfigurationList

impl InitializerConfiguration {
    /// watch individual changes to a list of InitializerConfiguration
    ///
    /// Use [`WatchAdmissionregistrationV1alpha1InitializerConfigurationListResponse`](./enum.WatchAdmissionregistrationV1alpha1InitializerConfigurationListResponse.html) to parse the HTTP response.
    ///
    /// # Arguments
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
    pub fn watch_admissionregistration_v1alpha1_initializer_configuration_list(
        field_selector: Option<&str>,
        include_uninitialized: Option<bool>,
        label_selector: Option<&str>,
        pretty: Option<&str>,
        resource_version: Option<&str>,
        timeout_seconds: Option<i64>,
        watch: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/admissionregistration.k8s.io/v1alpha1/watch/initializerconfigurations?");
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", label_selector);
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

/// Parses the HTTP response of [`InitializerConfiguration::watch_admissionregistration_v1alpha1_initializer_configuration_list`](./struct.InitializerConfiguration.html#method.watch_admissionregistration_v1alpha1_initializer_configuration_list)
#[derive(Debug)]
pub enum WatchAdmissionregistrationV1alpha1InitializerConfigurationListResponse {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent),
    Unauthorized,
    Other,
}

impl ::Response for WatchAdmissionregistrationV1alpha1InitializerConfigurationListResponse {
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
                Ok((WatchAdmissionregistrationV1alpha1InitializerConfigurationListResponse::Ok(result), byte_offset))
            },
            ::http::StatusCode::UNAUTHORIZED => Ok((WatchAdmissionregistrationV1alpha1InitializerConfigurationListResponse::Unauthorized, 0)),
            _ => Ok((WatchAdmissionregistrationV1alpha1InitializerConfigurationListResponse::Other, 0)),
        }
    }
}

// End admissionregistration.k8s.io/v1alpha1/InitializerConfiguration

impl<'de> ::serde::Deserialize<'de> for InitializerConfiguration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_initializers,
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
                            "initializers" => Field::Key_initializers,
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
            type Value = InitializerConfiguration;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct InitializerConfiguration")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_api_version: Option<String> = None;
                let mut value_initializers: Option<Vec<::v1_7::kubernetes::pkg::apis::admissionregistration::v1alpha1::Initializer>> = None;
                let mut value_kind: Option<String> = None;
                let mut value_metadata: Option<::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => value_api_version = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_initializers => value_initializers = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metadata => value_metadata = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(InitializerConfiguration {
                    api_version: value_api_version,
                    initializers: value_initializers,
                    kind: value_kind,
                    metadata: value_metadata,
                })
            }
        }

        deserializer.deserialize_struct(
            "InitializerConfiguration",
            &[
                "apiVersion",
                "initializers",
                "kind",
                "metadata",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for InitializerConfiguration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "InitializerConfiguration",
            0 +
            self.api_version.as_ref().map_or(0, |_| 1) +
            self.initializers.as_ref().map_or(0, |_| 1) +
            self.kind.as_ref().map_or(0, |_| 1) +
            self.metadata.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.api_version {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", value)?;
        }
        if let Some(value) = &self.initializers {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "initializers", value)?;
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
