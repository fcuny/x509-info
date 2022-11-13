# x509-info

[![Build Status](https://ci.fcuny.net/api/badges/fcuny/x509-info/status.svg)](https://ci.fcuny.net/fcuny/x509-info)

At this point it's pretty clear that I'll never remember the syntax for `openssl` to show various information about a certificate. At last I will not have to google for that syntax ever again.

## Usage

``` shell
> x509-info --help
Usage: x509-info [OPTIONS] <DOMAIN>

Arguments:
  <DOMAIN>
          Domain to check

Options:
  -p, --port <PORT>
          Port to check

          [default: 443]

  -f, --format <FORMAT>
          [default: short]

          Possible values:
          - short: Format the output as one line of plain text
          - long:  Format the output as plain text

  -h, --help
          Print help information (use `-h` for a summary)

  -V, --version
          Print version information
```

The default format will print a short message:

``` shell
> x509-info twitter.com
twitter.com is valid until Mon, 12 Dec 2022 15:59:59 -0800 (29 days left)
```

It's possible to get more details:

``` shell
> x509-info --format long twitter.com
certificate
 version: V3
 serial: 0d:e1:52:69:6b:2f:96:70:d6:c7:db:18:ce:1c:71:a0
 subject: C=US, ST=California, L=San Francisco, O=Twitter, Inc., CN=twitter.com
 issuer: C=US, O=DigiCert Inc, CN=DigiCert TLS RSA SHA256 2020 CA1
 validity
  not before    : Sun, 12 Dec 2021 16:00:00 -0800
  not after     : Mon, 12 Dec 2022 15:59:59 -0800
  validity days : 364
  remaining days: 29
 SANs:
  DNS:twitter.com
  DNS:www.twitter.com
```

## Notes

Could the same be achieved with a wrapper around `openssl` ? yes.
