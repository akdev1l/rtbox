use crate::rtbox::config::RtBoxConfig;
use crate::rtbox::engine::{
    RtBox,
    MockContainerEngine,
    RtBoxEngine,
};


#[tokio::test]
async fn rtbox_create_with_image () {

    let config: RtBoxConfig = RtBoxConfig::default();
    let mut container_engine: MockContainerEngine = MockContainerEngine::default();
    let rtbox_engine = RtBoxEngine::new(&config, &container_engine);

    let _created_rtbox = rtbox_engine.create(
        "alex",
        "test-image:latest",
    ).await;

    let _expected_rtbox = RtBox {
        name: "alex".to_string(),
        image: "test-image:latest".to_string(),
    };

    let _ = container_engine.expect_create();
}

#[tokio::test]
async fn rtbox_list_not_all() {

    let config: RtBoxConfig = RtBoxConfig::default();
    let mut container_engine: MockContainerEngine = MockContainerEngine::default();
    let rtbox_engine = RtBoxEngine::new(&config, &container_engine);

    let _ = rtbox_engine.list(None).await;

    _ = container_engine.expect_list();

}

#[tokio::test]
async fn rtbox_list_all() {

    let config: RtBoxConfig = RtBoxConfig::default();
    let mut container_engine: MockContainerEngine = MockContainerEngine::default();
    let rtbox_engine = RtBoxEngine::new(&config, &container_engine);

    let _ = rtbox_engine.list(Some(true)).await;

    _ = container_engine.expect_list();
}
