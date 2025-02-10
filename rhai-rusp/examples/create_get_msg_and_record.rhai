let body = rusp::get_builder()
    .with_params(["Device."])
    .with_max_depth(2)
    .build();
let msg = rusp::msg_builder()
    .with_msg_id("Foo")
    .with_body(body)
    .build();
let record = rusp::record_builder()
    .with_version("1.3")
    .with_to_id("proto::to")
    .with_from_id("proto::from")
    .with_no_session_context_payload(msg)
    .build();
