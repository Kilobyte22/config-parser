# A parser for configuration files.

## Syntax

The syntax is similar to the config of nginx and pulseaudio.

Here an example how an irc bot might be configured

```
# Connect to freenode
server freenode {
    connect irc.freenode.net 6697 tls;
    nick BleghBot blegh "I am BleghBot owned by MyAdmin";

    channel "#freenode";
    channel "#secret" mypassword;
    
    user MyAdmin {
        allow all;
    }

    user ShittySpammer {
        deny all;
    }
}
```

## API
The API is pretty simple:

```rust
extern crate config_parser;

let mut file = File::open("config.cfg").unwrap();

let cfg = config_parser::parse_file(file).unwrap();

for server in cfg.matches("server") {
    let s = Server::new(server.get(0));

    for channel in server.matches("channel") {
        s.add_channel(channel.get(0), channel.get_opt(1));
    }
}

```
