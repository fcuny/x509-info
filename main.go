package main

import (
	"flag"
	"fmt"
	"os"

	"github.com/fcuny/x509-info/internal/x509"
)

const usage = `Usage:
    x509-inf [DOMAIN]
`

func main() {
	flag.Usage = func() { fmt.Fprintf(os.Stderr, "%s\n", usage) }
	flag.Parse()

	if flag.NArg() != 1 {
		fmt.Fprintf(os.Stderr, "too many arguments: got %d, expected 1\n", flag.NArg())
		flag.Usage()
		os.Exit(1)
	}

	domain := flag.Arg(0)

	certs, err := x509.GetCertificates(domain, 443, false)
	if err != nil {
		os.Exit(1)
	}

	for _, cert := range certs {
		fmt.Printf("Issuer Name: %s\n", cert.Issuer)
		fmt.Printf("Expiry: %s \n", cert.NotAfter.Format("2006-January-02"))
		fmt.Printf("Common Name: %s \n", cert.Issuer.CommonName)
	}
}
