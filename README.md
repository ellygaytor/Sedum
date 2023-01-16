![cover](https://user-images.githubusercontent.com/33349740/135699024-5e643074-e58e-4b9a-bbaf-2ea1501b3ff6.png)

Sedum is a static site generator written in Rust. It can be used locally or with a service like Netlify to generate websites on the fly.

[![Check and Lint](https://github.com/ellygaytor/Sedum/actions/workflows/check_and_lint.yaml/badge.svg)](https://github.com/ellygaytor/Sedum/actions/workflows/check_and_lint.yaml) [![Security audit](https://github.com/ellygaytor/Sedum/actions/workflows/audit.yml/badge.svg)](https://github.com/ellygaytor/Sedum/actions/workflows/audit.yml) [![Release](https://github.com/ellygaytor/Sedum/actions/workflows/release.yml/badge.svg)](https://github.com/ellygaytor/Sedum/actions/workflows/release.yml) [![Netlify Status](https://api.netlify.com/api/v1/badges/23dd963b-38ec-4f1c-8d1a-7ab1fb373bc2/deploy-status)](https://app.netlify.com/sites/sedum/deploys)

## Usage

### Local

#### Prerequisites
You must have rust and cargo installed and available on the path.

#### Setup
  1. Run `git clone https://github.com/ellygaytor/Sedum.git`
  2. Prepend the appropriate yaml to your markdown files (prepending and appending `---`) (optional):
    - `title`
    - `description`
    - `language`
    - `author`
    - `list` ('True' or 'False')
#### Generation
  1. Run `cargo run [source] [result]`, setting the source directory and the directory you want the generated files to be placed in.

### Netlify

#### File Based

  1. Add `netlify.toml` to your git repository
  2. Change the build command and publish directory to fit your needs (optional)
  3. Prepend the appropriate yaml to your markdown files (prepending and appending ---) (optional)

#### Manual

##### Setup
  1. Set your build command to `wget -N https://github.com/ellygaytor/Sedum/releases/latest/download/sedum && chmod +x sedum && ./sedum [source] [result]`, setting the source directory and the directory you want the generated files to be placed in.
  2. Set your publish directory to `[result]` that you chose in step one.
  3. Prepend the appropriate yaml to your markdown files (prepending and appending `---`) (optional):
    - `title`
    - `description`
    - `language`
    - `author`
    - `list` ('True' or 'False')

##### Generation
  1. Place your markdown files in `[source]` that you set in step one of setup
  2. Push the changes, and Netlify will automatically download the latest version of Sedum, and generate the files.

## Options:
|Option|Usage|Description|
|:----|:----|:----|
|`-t` or `—timestamp`|`… -t …`|Add a timestamp (in seconds since epoch) in a comment to the generated HTML|
|`-m` or `-metadata`|`… -m …`|Generate a metadata file in the target directory that specifies the time at generation, operating system, and version of Sedum|

## Dynamic Replace:
|Usage|Description|
|:----|:----|
|`\|LIST\|`|Insert a HTML list of files with `list` enabled|
|`\|TIMESTAMP\|`|Insert the number of seconds since the epoch|
|`\|COPYRIGHT\|`|Insert a copyright notice for the author of the page or the default author in `settings`.|

## Settings File:
The settings file is placed in the source directory, and uses YAML.
|Usage|Description|
|:----|:----|
|`default_author`|The default author to be used if not set in the page options|