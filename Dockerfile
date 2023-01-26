# Rust as the base build image
FROM rust:1.66 as build

# 1. Create a new empty shell project
RUN USER=root cargo new --bin treeleaf
WORKDIR /treeleaf

# 2. Copy our manifests
COPY ./Cargo.lock ./Cargo.lock 
COPY ./Cargo.toml ./Cargo.toml 

# 3. Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# 4. Now that the dependency is built, copy your source code
COPY ./src ./src
# COPY ./docs ./docs
COPY README.md README.md

# 5. Build for release.
RUN rm ./target/release/deps/treeleaf*
RUN cargo build --release
# RUN rm -rf ./docs
RUN rm README.md

# # 6. Dockerfile3: Our final base (1.26GB)
# # FROM rust:1.61
# # 6. Dockerfile4: space-saving image variant (675MB)
# # FROM rust:1.61-slim-buster

# 6. Dockerfile5: Linux image without any rust (75.9MB)
FROM debian:buster-slim

# 7. Copy the build artifact from the build stage
COPY --from=build /treeleaf/target/release/treeleaf .

# 8. Set the startup command to run our binary
ENTRYPOINT ./treeleaf
# CMD ["./treeleaf"]
# # https://stackoverflow.com/a/53897608 | How to pass arguments to Shell Script through docker run
# # RUN chmod 755 /treeleaf/target/release/treeleaf
# # https://stackoverflow.com/questions/31523551/how-can-i-pass-arguments-to-a-docker-container-with-a-python-entry-point-script
# #
# # exec form 
# # ENTRYPOINT ["./treeleaf"]
# # shell form
