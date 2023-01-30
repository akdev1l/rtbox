use log::{info, warn};

use crate::rtbox::error::RtBoxError;
use crate::rtbox::config::RtBoxConfig;
use crate::rtbox::engine::{
    RtBox,
    MockContainerEngine,
    RtBoxEngine,
    Result,
};


#[tokio::test]
async fn rtbox_create_with_image () {
    let config = RtBoxConfig::default();
    let container_engine = MockContainerEngine::default();
    let rtbox_engine = RtBoxEngine::new(&config, &container_engine);

    let created_rtbox = rtbox_engine.create(
        "alex",
        "test-image:latest",
    ).await;

    let expected_rtbox = RtBox {
        name: "alex".to_string(),
        image: "test-image:latest".to_string(),
    };

    assert!(created_rtbox.unwrap() == expected_rtbox);
}
