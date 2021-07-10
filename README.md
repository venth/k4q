# Overview ![GitHub Workflow Status](https://img.shields.io/github/workflow/status/venth/kaf/Build) [![Work status](https://img.shields.io/badge/-under%20construction-orange)](https://img.shields.io/badge/status-under%20construction-orange)
The motivation for this project is simplification of my daily operations with kafka.
Often I: 
- check topics lag for partitions;
- offset on topics for all partitions;
- look for a specific record using key or some regex.

To perform these activities I use: [birdayz/kaf](https://github.com/birdayz/kaf) which is awesome and 
if only searching capabilities would be better then I wouldn't have motivation to
write my own tool.

# Acknowledges
## HowTos
- [project's layout](https://doc.rust-lang.org/cargo/guide/project-layout.html)
## Libraries
- [clap](https://github.com/clap-rs/clap) - provides good use & fill for the command line
- [mitsuhiko/indicatif](https://github.com/mitsuhiko/indicatif) - rendering progress bar and found records
