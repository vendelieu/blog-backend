# vendeli.eu-blog backend

Backend Api of my personal blog using rust + actix

# Instruction

1. Create a database

2. Rename .env.sample to .env and update properties (at least `DATABASE_URL`).

3. install libpq lib for postgres

```
sudo apt-get install libpq-dev
```

4. install diesel-cli through cargo install

```
cargo install diesel_cli --no-default-features --features postgres
```

5. run migrations:

```
diesel migration run
```

6. build with release profile:

```
cargo build --release
```

7. and run binary from console:

```
target/release/vendelieu-blog 
```
