## dank-paste

The World's Dankest Paste Bin!

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
```
location / {
    		proxy_pass http://127.0.0.1:{PORT NUMBER FOR dank-paste};
}
```

### Configuration

All server configuration is handled thru the default rocket config Rocket.toml
