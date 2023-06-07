# rstro
Selfhostable & retro-looking web GUI for the official ping host at [ping.qwq.sh](https://ping.qwq.sh).
I am planning on making this configurable for other hosts, but for now it's just for the official one.

### What's ping?
Ping is a barebones & anonymous chat platform as a web server built with SQLite and Rust. You can check it out [here](https://github.com/angelsflyinhell/ping).

# Host it yourself with Docker
```bash
docker run -d -p 80:8080 --name rstro qwqshq/rstro
```