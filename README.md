# Open data cli

Small little rust program that reads open data available in the Tampere region of Finland.

## usage

`cargo run` will compile and print out the CLI help text. That'll get you started.

## Open APIs

### road work and incidents

Data used can be found [here](https://data.tampere.fi/data/en_GB/dataset/tampereen-kaupungin-liikennetiedoterajapinta/resource/b3c2ce94-e12f-4e33-ae97-c6c23878e079)

## libs used

- [clap](https://docs.rs/clap/latest/clap/) for cli parsing
- [serde](https://serde.rs/) for json
- [reqwest](https://docs.rs/reqwest/latest/reqwest/) as an http client
- [tokio](https://tokio.rs/) for async business
