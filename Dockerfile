FROM rust:latest

WORKDIR /action

COPY . .

RUN cargo build --release

RUN cp target/release/github_classroom_spreadsheets_integration /action/github_classroom_spreadsheets_integration

ENTRYPOINT ["/action/github_classroom_spreadsheets_integration"]
