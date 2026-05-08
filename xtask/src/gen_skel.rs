use anyhow::{Context, Result, bail};
use libbpf_cargo::SkeletonBuilder;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use xshell::{Shell, cmd};

const BPF_OBJECT_SUFFIX: &str = ".bpf.o";

struct GeneratedRepo {
    name: String,
    modules: Vec<GeneratedModule>,
}

struct GeneratedModule {
    name: String,
    path: PathBuf,
}

pub fn run() -> Result<()> {
    let sh = Shell::new()?;
    cmd!(sh, "git submodule update --init --recursive")
        .run()
        .context("failed to update git submodules")?;

    let xtask_dir =
        PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").context("missing CARGO_MANIFEST_DIR")?);
    let project_dir = xtask_dir
        .parent()
        .with_context(|| format!("failed to find parent of {}", xtask_dir.display()))?;
    if !project_dir
        .join("crates")
        .join("bpf-skel")
        .join("src")
        .is_dir()
    {
        bail!(
            "failed to find project root from {}: expected a parent with crates/bpf-skel/src/",
            xtask_dir.display()
        );
    }

    let skel_dir = project_dir
        .join("crates")
        .join("bpf-skel")
        .join("src")
        .join("skel");
    if skel_dir.exists() {
        fs::remove_dir_all(&skel_dir).with_context(|| {
            format!(
                "failed to remove stale skeleton output directory {}",
                skel_dir.display()
            )
        })?;
    }
    fs::create_dir_all(&skel_dir).with_context(|| {
        format!(
            "failed to create skeleton output directory {}",
            skel_dir.display()
        )
    })?;

    let mut bpf_dirs = Vec::new();
    let bpf_root = xtask_dir.join("bpf");
    for entry in fs::read_dir(&bpf_root)
        .with_context(|| format!("failed to read BPF root directory {}", bpf_root.display()))?
    {
        let path = entry?.path();
        if !path.is_dir() {
            continue;
        }

        if !path.join("Makefile").is_file() {
            bail!(
                "BPF workspace {} does not contain a Makefile",
                path.display()
            );
        }

        bpf_dirs.push(path);
    }

    if bpf_dirs.is_empty() {
        bail!(
            "BPF root directory {} does not contain any workspaces",
            bpf_root.display()
        );
    }

    bpf_dirs.sort();

    let mut repos = Vec::with_capacity(bpf_dirs.len());
    for bpf_dir in bpf_dirs {
        let workspace_name = bpf_dir
            .file_name()
            .and_then(|name| name.to_str())
            .with_context(|| format!("invalid BPF workspace path: {}", bpf_dir.display()))?;

        let repo_skel_dir = skel_dir.join(workspace_name);
        fs::create_dir_all(&repo_skel_dir).with_context(|| {
            format!(
                "failed to create repository skeleton directory {}",
                repo_skel_dir.display()
            )
        })?;

        let objects = build_objects(&bpf_dir, workspace_name)?;
        let mut modules = Vec::with_capacity(objects.len());
        for object in objects {
            let module_name = bpf_object_name(&object)?;
            let output = repo_skel_dir.join(format!("{module_name}.skel.rs"));

            SkeletonBuilder::new()
                .obj(&object)
                .generate(&output)
                .with_context(|| {
                    format!("failed to generate Rust skeleton for {}", object.display())
                })?;

            modules.push(GeneratedModule {
                name: module_name,
                path: output,
            });
        }

        repos.push(GeneratedRepo {
            name: workspace_name.to_owned(),
            modules,
        });
    }

    write_module_index(&skel_dir.join("mod.rs"), &repos)
}

fn build_objects(bpf_dir: &Path, name: &str) -> Result<Vec<PathBuf>> {
    let sh = Shell::new()?;
    cmd!(sh, "make -C {bpf_dir}")
        .run()
        .with_context(|| format!("failed to run Makefile in {}", bpf_dir.display()))?;

    let output_dir = bpf_dir.join(".output");
    let mut objects = Vec::new();
    for entry in fs::read_dir(&output_dir)
        .with_context(|| format!("failed to read {}", output_dir.display()))?
    {
        let path = entry?.path();
        if path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.ends_with(BPF_OBJECT_SUFFIX))
        {
            objects.push(path);
        }
    }
    objects.sort();

    if objects.is_empty() {
        bail!(
            "BPF workspace `{name}` Makefile did not produce any *{BPF_OBJECT_SUFFIX} files in {}",
            output_dir.display()
        );
    }

    Ok(objects)
}

fn bpf_object_name(object: &Path) -> Result<String> {
    let file_name = object
        .file_name()
        .and_then(|name| name.to_str())
        .with_context(|| format!("invalid BPF object path: {}", object.display()))?;

    file_name
        .strip_suffix(BPF_OBJECT_SUFFIX)
        .map(ToOwned::to_owned)
        .ok_or_else(|| {
            anyhow::anyhow!(
                "BPF object does not end with {BPF_OBJECT_SUFFIX}: {}",
                object.display()
            )
        })
}

fn write_module_index(path: &Path, repos: &[GeneratedRepo]) -> Result<()> {
    let mut contents = String::from("// @generated by xtask gen-skel\n\n");
    for repo in repos {
        contents.push_str(&format!("pub mod {} {{\n", repo.name));
        for module in &repo.modules {
            let module_path = module
                .path
                .file_name()
                .and_then(|name| name.to_str())
                .with_context(|| format!("invalid skeleton path: {}", module.path.display()))?;
            contents.push_str(&format!(
                "    #[path = {:?}]\n    pub mod {};\n",
                module_path, module.name
            ));
        }
        contents.push_str("}\n\n");
    }

    fs::write(path, contents)
        .with_context(|| format!("failed to write generated module index {}", path.display()))?;
    Ok(())
}
