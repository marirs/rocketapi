API REST Server using Rocket.rs
--------------------------------
[![Build Status](https://travis-ci.com/marirs/rocketapi.svg?branch=master)](https://travis-ci.com/marirs/rocketapi)

A Skeleton API Rest server using [Rocket](https://rocket.rs/) with the backend database as [MongoDB](https://www.mongodb.com/).

### Features
- Custom config file defining:
  - server host ip and port to listen
  - enable/disable ssl with ssl cert auto generation
  - mongodb configurations
- Use the `x-api-key` header to validate `API Keys`
- `Restrict` a client connecting IP Addresses to the endpoints using `Allow ACL`
- `Restrict` endpoints using the `Deny ACL`
- `Rate limiter` to throttle incoming requests to endpoints
- Extend this rocketapi server's boiler plate; with your endpoints

![architecture](docs/rocketapi.png "Architecture")

---
### Compile

```bash
cargo build --release
```

- Sample config file is available at `config.sample.yml`

### Starting the server
- using a config file
```bash
./target/release/rocketapi runserver -f <CONFIG_PATH>
```
- using a total default configuration
```bash
./target/release/rocketapi runserver
```

- When the server is enabled to start with SSL, the certs will be generated in the same folder called `private`.
---
### Creating your first user
```bash
./target/release/rocketapi createsuperuser -e <EMAIL> -f <CONFIG_PATH>
```

### Configs  
The `configs` folder has configurations to start the server as a service and nginx config to server this rocketapi server in reverse proxy mode.
- Start the rocketapi server as a service
```text
1) copy the configs/rocketapi.service to /etc/systemd/system folder
2) systemctl enable rocketapi
3) systemctl start rocketapi
```

- Then you can copy the `configs/nginx-server-config` to `/etc/nginx/sites-enabled` to acces the rocketapi server via nginx.

---

If you need a python version, a python fastapi version can be found [here](https://github.com/marirs/fastapi-boilerplate).

---
### Contribution

Feel free to make pull requests and make this better and/or contribute to its features.

---
Licensed under the Apache License, Version 2.0
