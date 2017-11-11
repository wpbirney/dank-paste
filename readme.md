## dank-paste

The World's Dankest Paste Bin!

If *anyone* would like to write this readme feel free!

### Quick-Start

dank-paste requires rocket, therefor currently requiring a nightly rust compiler,
I recommend using [rustup](https://www.rustup.rs/) to get nightly and stay up to date.
The vanilla build is currently hosted at [ganja.ml](https://ganja.ml), so go check it out

1. Get dank-paste source
   ```
   git clone https://github.com/wpbirney/dank-paste
   ```
2. Run with cargo (default will launch server in debug mode)
   ```
   cd dank-paste
   cargo run
   ```

### Deployment

When deploying to a production server it is recommended to put dank-paste behind nginx using proxy_pass
Its also recommended when using nginx to use proxy_set_header, otherwise the fuckn' rate limiting wont work
```
location / {
    		proxy_pass http://127.0.0.1:{PORT NUMBER FOR dank-paste};
			proxy_set_header real-ip $remote_addr;
}
```

### Configuration

All server configuration is handled thru the default rocket config Rocket.toml
