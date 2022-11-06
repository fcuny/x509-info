# x509-info

[![Build Status](https://ci.fcuny.net/api/badges/fcuny/x509-info/status.svg)](https://ci.fcuny.net/fcuny/x509-info)

At this point it's pretty clear that I'll never remember the syntax for `openssl` to show various information about a certificate. At last I will not have to google for that syntax ever again.

``` shell
$ x509-info github.com
        Subject: CN=github.com O=GitHub, Inc. L=San Francisco
        Issuer:  CN=DigiCert TLS Hybrid ECC SHA384 2020 CA1 O=DigiCert Inc L=
        DNS Names: github.com, www.github.com
        Validity Period
                Not before: 2022-03-14T17:00:00-07:00
                Not After:  2023-03-15T16:59:59-07:00
```

Could the same be achieved with a wrapper around `openssl` ? yes.
