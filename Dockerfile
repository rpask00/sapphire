FROM rust:1.62 as build

# create a new empty shell project
RUN USER=root cargo new sapphire
WORKDIR /sapphire

# copy over your manifests
COPY ./Cargo.* .
COPY ./.env ./.env

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src
COPY ./assets ./assets

# build for release
RUN rm ./target/release/deps/sapphire*
RUN cargo build --release


# our final base
FROM rust:1.61-slim

# copy the build artifact from the build stage
COPY --from=build /sapphire/target/release/sapphire .
COPY --from=build /sapphire/.env .

# set the startup command to run your binary
CMD ["./sapphire"]
