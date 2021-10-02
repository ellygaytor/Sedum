![cover](https://user-images.githubusercontent.com/33349740/135699024-5e643074-e58e-4b9a-bbaf-2ea1501b3ff6.png)

Sedum is a static site generator written in Rust. It can be used locally or with a service like Netlify to generate websites on the fly.

![GitHub Workflow Status](https://img.shields.io/github/workflow/status/ellygaytor/Sedum/Rust)

## Usage

### Local

#### Prerequisites
You must have rust and cargo installed and available on the path.

#### Setup
1. Run `git clone https://github.com/ellygaytor/Sedum.git`
2. Create a folder in the directory you cloned to called `source`
3. Prepend the appropriate yaml to your markdown files:
  - `title`
  - `description`
  - `language`
  - `author`
4. Move your markdown files into `source`

#### Generation
1. Run `cargo run` in the parent folder of `source`
2. Move the contents of `result` to the appropriate location
