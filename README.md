## solaSearch

A project to help catalog and relate various early Church texts and documents. These documents are hard to find and discover across the world wide web, the aim to not only to gather them into a single spot, but to present them in such a way as they can be related together and searched in a user friendly way.

### Rust, ElasticSearch, and Postgres.
Implementation in Rust, ElasticSearch, and Postgres.


The idea is to eventualy write an API and Web UI front end to process and present a trove of information relating books and documents together. Enabling simple search and relationship drilling.
<img width="810" alt="Screenshot 2023-02-05 at 8 20 19 PM" src="https://user-images.githubusercontent.com/34192225/216868411-e306ee59-0da0-4d7e-b4dc-ef7d62a55e7c.png">

### Docker and running the code.
First, build the Dockerfile with the following command `docker build . --tag=gutenberg-rs`

Next, to drop into the Dockerfile run `docker run -it gutenberg-rs /bin/bash`

To execute the `main` function call `cargo run` inside the Dockerfile.
