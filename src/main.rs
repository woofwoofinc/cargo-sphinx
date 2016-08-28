extern crate cargo;
extern crate clap;
#[macro_use]
extern crate quick_error;
extern crate term;
extern crate toml;

use std::fs::File;
use std::path::Path;
use std::process::exit;

use clap::{App, ArgMatches, SubCommand};
use cargo::util::Config as CargoConfig;
use cargo::core::MultiShell;

mod config;
mod error;
mod cmd;
mod git;

use cmd::call;
use config::Config;
use error::FatalError;
use term::color;

fn build(docs_path: &str, dry_run: bool) -> Result<(), FatalError> {
    try!(shell.say("Building Sphinx docs.", color::BLUE));
    try!(call(vec!["make", "clean", "html"], docs_path, dry_run));

    // A `.nojekyll` file prevents GitHub from ignoring Sphinx CSS files.
    let nojekyll = Path::new(docs_path).join("_build/html/.nojekyll");
    if !nojekyll.exists() {
        try!(File::create(nojekyll));
    }

    Ok(())
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

    try!(git::init(&docs_build_path, dry_run));
    try!(git::add_all(&docs_build_path, dry_run));
    try!(git::commit_all(&docs_build_path, commit_msg, sign, dry_run));
    let remote = try!(git::remote_get_url(push_remote));

    let mut refspec = String::from("master:");
    refspec.push_str(push_branch);

    git::force_push(docs_path, remote.trim(), &refspec, dry_run)
}

fn execute(args: &ArgMatches, cargo_config: &CargoConfig) -> Result<i32, FatalError> {
    try!(cargo_config.configure_shell(args.occurrences_of("verbose") as u32,
                                      Some(args.is_present("quiet")),
                                      &args.value_of("color").map(|s| String::from(s))));

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
    try!(build(docs_path, &mut *cargo_config.shell(), dry_run));
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
            .arg_from_usage("-v, --verbose 'Use verbose output.'")
            .arg_from_usage("-q, --quiet 'Less output printed to stdout.'")
            .arg_from_usage("--color 'Coloring: auto, always, never.'")
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

    let cargo_config = CargoConfig::default().expect("Unable to get config");

    if let Some(sphinx_matches) = matches.subcommand_matches("sphinx") {
        match execute(sphinx_matches, &cargo_config) {
            Ok(code) => exit(code),
            Err(e) => {
                cargo_config.shell().error(format!("Fatal: {}", e)).unwrap();
                exit(128);
            }
        }
    }
}
