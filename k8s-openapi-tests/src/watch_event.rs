#[test]
fn watch_pods() {
	k8s_if_1_7! {
		use ::k8s_openapi::v1_7::kubernetes::pkg::api::v1 as api;
	}
	k8s_if_1_8! {
		use ::k8s_openapi::v1_8::api::core::v1 as api;
	}
	k8s_if_1_9! {
		use ::k8s_openapi::v1_9::api::core::v1 as api;
	}
	k8s_if_1_10! {
		use ::k8s_openapi::v1_10::api::core::v1 as api;
	}
	k8s_if_1_11! {
		use ::k8s_openapi::v1_11::api::core::v1 as api;
	}

	let client = ::Client::new().expect("couldn't create client");

	k8s_if_le_1_7! {
		let request =
			api::Pod::watch_core_v1_namespaced_pod_list("kube-system", None, None, None, None, None, None, None);
	}
	k8s_if_ge_1_8! {
		let request =
			api::Pod::watch_core_v1_namespaced_pod_list("kube-system", None, None, None, None, None, None, None, None, None);
	}
	let request = request.expect("couldn't watch pods");
	let response = client.execute(request).expect("couldn't watch pods");
	let pod_watch_events =
		::get_multiple_values(response, |response, status_code, _| match response {
			api::WatchCoreV1NamespacedPodListResponse::Ok(pod_watch_event) =>
				Ok(::ValueResult::GotValue(pod_watch_event)),
			other => Err(format!("{:?} {}", other, status_code).into()),
		}).expect("couldn't watch pods");

	let addon_manager_pod =
		pod_watch_events
		.map(|pod_watch_event| pod_watch_event.expect("couldn't get pod watch event"))
		.filter_map(|pod_watch_event| {
			if pod_watch_event.type_ != "ADDED" {
				return None;
			}

			let pod: api::Pod = ::serde_json::from_value(pod_watch_event.object.0).expect("couldn't re-deserialize pod watch event object");
			if pod.metadata.as_ref().and_then(|metadata| metadata.name.as_ref()).map_or(false, |name| name.starts_with("kube-addon-manager-")) {
				Some(pod)
			}
			else {
				None
			}
		})
		.next().expect("couldn't find addon-manager pod");

	let addon_manager_container_spec =
		addon_manager_pod
		.spec.expect("couldn't get addon-manager pod spec")
		.containers
		.into_iter().next().expect("couldn't get addon-manager container spec");
	assert_eq!(addon_manager_container_spec.name, "kube-addon-manager");

	let addon_manager_pod_status = addon_manager_pod.status.expect("couldn't get addon-manager pod status");
	assert_eq!(addon_manager_pod_status.phase, Some("Running".to_string()));
}
