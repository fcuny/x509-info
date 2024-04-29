# x509-info

At this point it's pretty clear that I'll never remember the syntax for `openssl` to show various information about a certificate. At last I will not have to google for that syntax ever again.

## Usage

``` shell
$ Usage: x509-info [OPTIONS] <DOMAIN>

Arguments:
  <DOMAIN>
          Domain to check

Options:
  -p, --port <PORT>
          Port to check

          [default: 443]

  -i, --insecure
          Accept invalid certificate

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
$ x509-info twitter.com
twitter.com: Mon, 11 Dec 2023 15:59:59 -0800 (257 days left)
```

It's possible to get more details:

``` shell
$ x509-info --format=long twitter.com
certificate
 version: V3
 serial: 0a:2c:01:b8:2b:5d:47:73:9a:5a:01:1a:6f:dc:1a:20
 subject: C=US, ST=California, L=San Francisco, O=Twitter, Inc., CN=twitter.com
 issuer: C=US, O=DigiCert Inc, CN=DigiCert TLS RSA SHA256 2020 CA1
 validity
  not before    : Sat, 10 Dec 2022 16:00:00 -0800
  not after     : Mon, 11 Dec 2023 15:59:59 -0800
  validity days : 365
  remaining days: 257
 SANs:
  DNS:twitter.com
  DNS:www.twitter.com
```

You can also check expired certificates:

``` shell
$ x509-info --insecure expired.badssl.com
<no name>: Sun, 12 Apr 2015 16:59:59 -0700 (it expired -2907 days ago)
```

## Notes

Could the same be achieved with a wrapper around `openssl` ? yes.
