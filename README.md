# x509-info

At this point it's pretty clear that I'll never remember the syntax for `openssl` to show various information about a certificate. At last I will not have to google for that syntax ever again.

## Usage

``` shell
Usage:
    x509-info [DOMAIN]
    x509-info (-f long) [DOMAIN]

Options:
    -f, --format      Format the result. Valid values: short, long. Default: short
    -i, --insecure    Skip the TLS validation. Default: false
    -p, --port        Specify the port. Default: 443
    -v, --version     Print version information
    -h, --help        Print this message
```

The default format will print a short message:
``` shell
$ ./bin/x509-info github.com
github.com, valid until Thu, 14 Mar 2024 23:59:59 UTC (86 days left)
```

It's possible to get more details:
``` shell
$ ./bin/x509-info -f long github.com
certificate
  version: 3
  serial: 17034156255497985825694118641198758684
  subject: github.com
  issuer: DigiCert TLS Hybrid ECC SHA384 2020 CA1

validity:
  not before: Tue, 14 Feb 2023 00:00:00 UTC
  not after: Thu, 14 Mar 2024 23:59:59 UTC
  validity days: 394
  remaining days: 86

SANs:
  • github.com
  • www.github.com
```

You can also check expired certificates:
``` shell
$ ./bin/x509-info -i expired.badssl.com
*.badssl.com, not valid since Sun, 12 Apr 2015 23:59:59 UTC (expired 3172 days ago)
```

## Notes

Could the same be achieved with a wrapper around `openssl` ? yes.
