#[macro_use]
extern crate quick_error;
extern crate toml;
extern crate clap;

use std::process::exit;

use clap::{App, ArgMatches, SubCommand};

mod config;
mod error;
mod cmd;
mod git;

use cmd::call;
use config::Config;
use error::FatalError;

fn build(docs_path: &str, dry_run: bool) -> Result<bool, FatalError> {
    println!("Building Sphinx docs.");
    call(vec!["make", "clean", "html"], docs_path, dry_run)
}

fn publish(docs_path: &str,
           commit_msg: &str,
           sign: bool,
           push_remote: &str,
           push_branch: &str,
           dry_run: bool)
           -> Result<bool, FatalError> {
    println!("Publishing Sphinx docs to GitHub Pages.");
    let docs_build_path = format!("{}/_build/html", docs_path);

    // A `.nojekyll` file prevents Github from ignoring Sphinx CSS files.
    try!(call(vec!["touch", ".nojekyll"], &docs_build_path, dry_run));

    try!(git::init(&docs_build_path, dry_run));
    try!(git::add_all(&docs_build_path, dry_run));
    try!(git::commit_all(&docs_build_path, commit_msg, sign, dry_run));
    let remote = try!(git::remote_get_url(push_remote));

    let mut refspec = String::from("master:");
    refspec.push_str(push_branch);

    try!(git::force_push(docs_path, remote.trim(), &refspec, dry_run));

    // Clean up.
    call(vec!["rm", "-fr", ".nojekyll", ".git"],
         &docs_build_path,
         dry_run)
}

fn execute(args: &ArgMatches) -> Result<i32, FatalError> {
    let config: Config = try!(Config::from("Cargo.toml"));

    // Find parameters or use defaults.
    let dry_run = args.is_present("dry-run");

    let docs_path = args.value_of("docs-path")
        .or_else(|| config.get_str(config::DOCS_PATH))
        .unwrap_or("docs");

    let push = args.is_present("push");

    let commit_msg = args.value_of("commit-message")
        .or_else(|| config.get_str(config::COMMIT_MESSAGE))
        .unwrap_or("(cargo-sphinx) Generate docs.");

    let sign = args.is_present("sign") || config.get_bool(config::SIGN_COMMIT).unwrap_or(false);

    let push_remote = args.value_of("push-remote")
        .or_else(|| config.get_str(config::PUSH_REMOTE))
        .unwrap_or("origin");

    let push_branch = args.value_of("push-branch")
        .or_else(|| config.get_str(config::PUSH_BRANCH))
        .unwrap_or("gh-pages");

    // Generate and publish documentation.
    try!(build(docs_path, dry_run));
    if push {
        try!(publish(docs_path,
                     commit_msg,
                     sign,
                     push_remote,
                     push_branch,
                     dry_run));
    }

    Ok(0)
}

fn main() {
    let matches = App::new("cargo")
        .subcommand(SubCommand::with_name("sphinx")
            .version(env!("CARGO_PKG_VERSION"))
            .author("Ning Sun <sunng@about.me>")
            .author("Woof Woof, Inc.")
            .about("Cargo subcommand for building and publishing Sphinx documentation to GitHub \
                    Pages.")
            .arg_from_usage("--dry-run 'Print commands to execute instead of running.'")
            .arg_from_usage("-p, --push 'Push generated documentation to git remote.'")
            .arg_from_usage("-s, --sign 'Sign the git commit.'")
            .arg_from_usage("--docs-path=[STRING] 'Path of Sphinx documentation to build. \
                             Defaults to `docs` if not specified.'")
            .arg_from_usage("--commit-message=[STRING] 'Commit message for the documentation \
                             change. Defaults to `(cargo-sphinx) Generate docs.` if not \
                             specified.'")
            .arg_from_usage("--push-remote=[STRING] 'Git remote to push. \
                             Defaults to `origin` if not specified.'")
            .arg_from_usage("--push-branch=[STRING] 'Git branch to push documentation on. \
                             Defaults to `gh-pages` if not specified.'"))
        .get_matches();

    if let Some(sphinx_matches) = matches.subcommand_matches("sphinx") {
        match execute(sphinx_matches) {
            Ok(code) => exit(code),
            Err(e) => {
                println!("Fatal: {}", e);
                exit(128);
            }
        }
    }
}
