![cover](https://user-images.githubusercontent.com/33349740/135699024-5e643074-e58e-4b9a-bbaf-2ea1501b3ff6.png)

Sedum is a static site generator written in Rust. It can be used locally or with a service like Netlify to generate websites on the fly.

![GitHub Workflow Status](https://img.shields.io/github/workflow/status/ellygaytor/Sedum/Rust) [![codecov](https://codecov.io/gh/ellygaytor/Sedum/branch/main/graph/badge.svg?token=7QNP00NYOC)](https://codecov.io/gh/ellygaytor/Sedum) [![Netlify Status](https://api.netlify.com/api/v1/badges/23dd963b-38ec-4f1c-8d1a-7ab1fb373bc2/deploy-status)](https://app.netlify.com/sites/sedum/deploys)


## Usage

### Local

#### Prerequisites
You must have rust and cargo installed and available on the path.

#### Setup
1. Run `git clone https://github.com/ellygaytor/Sedum.git`
2. Prepend the appropriate yaml to your markdown files:
  - `title`
  - `description`
  - `language`
  - `author`
  - `list` ('True' or 'False') [Optional]
#### Generation
1. Run `cargo run [source] [target]`, setting the source directory and the directory you want the generated files to be placed in

### Netlify

#### Setup
1. Set your build command to `wget https://github.com/ellygaytor/Sedum/releases/latest/download/sedum-netlify && chmod +x sedum-netlify && ./sedum-netlify [source] [result]`, setting the source directory and the directory you want the generated files to be placed in.
2. Set your publish directory to `[target]` that you chose in step one.
3. Prepend the appropriate yaml to your markdown files:
  - `title`
  - `description`
  - `language`
  - `author`
  - `list` ('True' or 'False') [Optional]

#### Generation
1. Place your markdown files in `[source]` that you set in step one of setup
2. Push the changes, and Netlify will automatically download the latest version of Sedum, and generate the files.
