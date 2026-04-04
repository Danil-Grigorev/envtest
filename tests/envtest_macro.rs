#![cfg(feature = "kube")]

use envtest::envtest;

#[envtest]
#[tokio::test]
async fn create(client: kube::Client) -> Result<(), Box<dyn std::error::Error>> {
    let version = client.apiserver_version().await?;
    assert!(!version.git_version.is_empty());
    Ok(())
}

fn env() -> envtest::Environment {
    envtest::Environment {
        binary_assets_settings: envtest::BinaryAssetsSettings {
            download_binary_assets: true,
            ..Default::default()
        },
        ..Default::default()
    }
}

#[envtest(environment = env())]
#[tokio::test]
async fn create_with_custom_environment(
    client: kube::Client,
) -> Result<(), Box<dyn std::error::Error>> {
    let version = client.apiserver_version().await?;
    assert!(!version.git_version.is_empty());
    Ok(())
}
