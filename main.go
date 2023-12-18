package main

import (
	"crypto/tls"
	"crypto/x509"
	"embed"
	"flag"
	"fmt"
	"html/template"
	"os"
	"time"
)

const usage = `Usage:
    x509-inf [DOMAIN]
`

func main() {

	flag.Usage = func() { fmt.Fprintf(os.Stderr, "%s\n", usage) }

	var (
		portFlag         int
		outputFormatFlag string
		insecureFlag     bool
	)

	flag.IntVar(&portFlag, "port", 443, "Port to check")
	flag.StringVar(&outputFormatFlag, "format", "short", "Format the output")
	flag.BoolVar(&insecureFlag, "insecure", false, "Whether to bypass secure flag checks")

	flag.Parse()

	if flag.NArg() != 1 {
		fmt.Fprintf(os.Stderr, "too many arguments: got %d, expected 1\n", flag.NArg())
		flag.Usage()
		os.Exit(1)
	}

	domain := flag.Arg(0)

	certs, err := getCertificates(domain, portFlag, insecureFlag)
	if err != nil {
		fmt.Fprintf(os.Stderr, "error: %v\n", err)
		os.Exit(1)
	}

	switch outputFormatFlag {
	case "long":
		printLong(certs)
	default:
		printShort(certs)
	}
}

func getCertificates(domain string, port int, insecureSkipVerify bool) ([]*x509.Certificate, error) {
	conf := &tls.Config{
		InsecureSkipVerify: insecureSkipVerify,
	}

	remote := fmt.Sprintf("%s:%d", domain, port)

	conn, err := tls.Dial("tcp", remote, conf)
	if err != nil {
		return nil, fmt.Errorf("failed to get the certificate for %s: %v", remote, err)
	}

	defer conn.Close()

	certs := conn.ConnectionState().PeerCertificates
	return certs, nil
}

func printShort(certs []*x509.Certificate) {
	cert := certs[0]

	now := time.Now()
	remainingDays := cert.NotAfter.Sub(now)

	if remainingDays > 0 {
		fmt.Printf("%s: %s (%d days left)\n", cert.Subject.CommonName, cert.NotAfter.Format("01/02/2006"), int(remainingDays.Hours()/24))
	} else {
		fmt.Printf("%s: %s (expired %d days ago)\n", cert.Subject.CommonName, cert.NotAfter.Format("01/02/2006"), int(remainingDays.Abs().Hours()/24))
	}
}

//go:embed "long.tmpl"
var tmplLong embed.FS

func printLong(certs []*x509.Certificate) {

	funcMap := template.FuncMap{
		"validFor": func(before, after time.Time) int {
			validForDays := after.Sub(before)
			return int(validForDays.Hours() / 24)
		},
		"remainingDays": func(notAfter time.Time) int {
			now := time.Now()
			remainingDays := notAfter.Sub(now)
			return int(remainingDays.Hours() / 24)
		},
	}

	tmpl, err := template.New("long.tmpl").Funcs(funcMap).ParseFS(tmplLong, "*")
	if err != nil {
		panic(err)
	}

	err = tmpl.Execute(os.Stdout, certs[0])
	if err != nil {
		panic(err)
	}
}
