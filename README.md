# rtbox: a rust implemention of toolbx

This aims to be a third implementation of a 'toolbox' runtime. We take inspiration
from the original as well as distrobox and try to combine the image-baked approach
with the feature-completeness of `distrobox` to try to provide with the best of both
worlds: a performant, feature complete toolbox runtime that is self-contained and easy
to install.

## Goals

1. Compatible: we will strive to be compatible with `toolbx` and `distrobox` wherever possible.
2. Performant: we will keep performance at the forefront as we believe that performance can hamper usability and adoption of containerized technologies.
3. Ephemeral: we will enable users to keep ephemeral workflows with a configuration-first mindset
4. Simple: we will keep the application self-contained and easy to use, advanced features should not get in the way of common usage patterns.

## Non-Goals

1. We will not be implementing anything that is not supported by the upstream podman project.
2. We will not implement extensive sandboxing.

## Installation

Currently this project is in **very early stages** of development. It doesn't actually do much of anything but it provides
a toolbx-compatible CLI implementation. We are using rust so to build this project we can easily do it by following these steps:

1. Clone the repo
2. run `./util/cargo build`

The `./util/cargo` script is a wrapper that will fetch cargo from dockerhub and run the build process. It will also create a folder
`${HOME}/.cache/cargo` to cache crates for future builds.

## Usage

This project is currently functionally useless but if you are interested I will leave a sample of the help messages:

```
$ rtbox --help
A rust implementation of toolbx

Usage: rtbox [OPTIONS] <COMMAND>

Commands:
  create  Create a rtbox container
  rm      Remove a rtbox container
  rmi     Remove a rtbox container image
  run     Execute a command inside a rtbox container
  enter   Enter into a new shell session inside a rtbox container
  list    List all rtbox containers
  export  Export an application, service or binary from a rtbox container to the host
  help    Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose <VERBOSE>      
  -y, --assume-yes             Automatically answer yes to all questions
      --log-level <LOG_LEVEL>  Set the logging level [default: info] [possible values: info, warn, error, debug, all]
  -f, --format <FORMAT>        Set the output format [default: human] [possible values: json, human]
      --dry-run                Do not actually execute API calls
  -h, --help                   Print help information
```

## Acknowledgements

1. Inspiration taken from the [toolbx project](https://github.com/containers/toolbox)
2. A lot of study conducted on [distrobox project](https://github.com/89luca89/distrobox)
3. We naturally depend on the [podman project](https://github.com/containers/podman) for all our container-related needs
4. And many thanks to the [podman-api rust crate](https://crates.io/crates/podman-api) which makes the implementation of this whole thing easier

## License 

The code here is released under the GPLv3. You can find a copy of the license [here](https://www.gnu.org/licenses/gpl-3.0.en.html) or in the repo itself.

## FAQ


#### Q: Why creating this?

A: I was interested in learning rust and needed a problem domain where I already had experience. Neither `toolbox` nor `distrobox` fit my workflow quite well so I though I would take a stab at it.

#### Q: What benefits will I get if I use `rtbox` instead?

A: Currently none as there no functionality. In the future I plan to implement features that aren't available in other implementations namely: a configuration file format for quickly replicating toolbox setups and a backup system are on my mind.

#### Q: Configuration file format? What do you mean?

A: This is for my own reference :) - I have some thoughts explaining what I mean on this [issue](https://github.com/containers/toolbox/issues/1018)
