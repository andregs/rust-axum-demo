# Docker Builder for Rust apps

This image leverages some tricks to speed-up the docker build process.

Tutorial available at:

https://www.lpalmieri.com/posts/fast-rust-docker-builds/

We don't execute commands here, but other dockerfiles in the project reference this as their base image.
