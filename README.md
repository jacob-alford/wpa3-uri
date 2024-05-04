# wpa3-uri

A rust cli utility for generating WPA3 URIs according to [the specification](https://www.wi-fi.org/system/files/WPA3%20Specification%20v3.1.pdf)

## Run the Thing

```console
cargo run -- --help
```

Or, for example, if your WPA3 exclusive wifi name is 👖:

```console
cargo run -- --ssid 👖 -p my-password --encryption-transition 1
```

...which results in:

```
WIFI:T:WPA;R:1;S:👖;P:my-password;;
```

## TODO:

- Public Key
- Id
- Validation
- Percent encoding (currently nothing is encoded)
