use std::path::Path;
use std::process::Command;
use anyhow::Context;
use tera::{Context as TeraContext, Tera};

fn main() -> anyhow::Result<()> {
    let tera = Tera::new("src/templates/*")?;
    let container_tag = "foo";

    // Binary search
    let mut min = 3;
    let mut max_exclusive = 500;
    while min < max_exclusive {
        let current_layer_count = (min + max_exclusive) / 2;
        assert!(current_layer_count > 2, "The minimum amount of layers is 3 (determined by the base image)");
        let file_count = current_layer_count - 2;

        println!("Trying layer count = {current_layer_count}");

        let path = format!("target/dockerfiles/{file_count}");
        render_dockerfile(&tera, path.as_ref(), file_count).context("failed to render Dockerfile")?;

        build_container(&path, container_tag)?;
        let succeeded = run_container(container_tag)?;

        if succeeded {
            min = current_layer_count + 1;
            println!("-> Success!");
        } else {
            max_exclusive = current_layer_count;
            println!("-> Failure!");
        }
    }

    println!("Max layer count = {}", min - 1);
    Ok(())
}

fn render_dockerfile(tera: &Tera, path: &Path, file_count: u32) -> anyhow::Result<()> {
    let mut context = TeraContext::new();
    context.insert("file_count", &file_count);
    let rendered = tera.render("dockerfile.j2", &context)?;

    std::fs::create_dir_all(path)?;
    std::fs::write(path.join("Dockerfile"), &rendered).context("failed to write")?;

    Ok(())
}

fn build_container(path: &str, tag: &str) -> anyhow::Result<()> {
    let status = Command::new("docker")
        .arg("build")
        .arg("-t")
        .arg(tag)
        .arg(path)
        .status()?;

    assert!(status.success(), "fatal error: docker build failed");
    Ok(())
}

fn run_container(tag: &str) -> anyhow::Result<bool> {
    let status = Command::new("docker")
        .arg("run")
        .arg("--rm")
        .arg(tag)
        .status()?;

    Ok(status.success())
}
