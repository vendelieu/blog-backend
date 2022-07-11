# vendeli.eu-blog backend

Backend Api of my personal blog using rust + actix

# Instruction

1. create secret key by command:

```
head -c32 /dev/urandom > src/secret.key
```

2. Create a database

3. Rename .env.sample to .env and update properties (at least `DATABASE_URL`).

4. install libpq lib for postgres

```
sudo apt-get install libpq-dev
```

5. install diesel-cli through cargo install

```
cargo install diesel_cli --no-default-features --features postgres
```

6. run migrations:

```
diesel migration run
```

7. build with release profile:

```
cargo build --release
```

8. and run binary from console:

```
target/release/vendelieu-blog 
```
