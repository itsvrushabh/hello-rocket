###basic steps to create a new rocket project

1. cargo init project-name --bin //To create the bin to run the project. rather than lib
2. cd project-name
3. add rocket dependency in Cargo.toml // below dependency to add "rocket = "0.5.1"
4. cargo run

### To always run the project we need watcher
1. cargo install cargo-watch
2. cargo watch -x 'run'
