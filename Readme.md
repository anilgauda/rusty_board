Creating Trello board using Rust as Backend

Setup
1. Docker Commands to install Postgres
    * Change Directory to DockerSetup
    * Build Docker Image: docker build -t rustyboard-db ./
    * Add Docker Image: docker images -a
    * Docker Run the Container: docker run -d --name my-postgresdb-container -p 5432:5432 rustyboard-db
