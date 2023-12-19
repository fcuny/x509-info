package main

import (
	"crypto/tls"
	"crypto/x509"
	"flag"
	"fmt"
	"html/template"
	"os"
	"time"
)

const usage = `Usage:
    x509-info [DOMAIN]
    x509-info (-f long) [DOMAIN]

Options:
    -f, --format      Format the result. Valid values: short, long. Default: short
    -i, --insecure    Skip the TLS validation. Default: false
    -p, --port        Specify the port. Default: 443
    -v, --version     Print version information
    -h, --help        Print this message
`

var (
	Version, BuildDate string
)

func main() {

	flag.Usage = func() { fmt.Fprintf(os.Stderr, "%s\n", usage) }

	var (
		portFlag         int
		outputFormatFlag string
		insecureFlag     bool
		versionFlag      bool
	)

	flag.IntVar(&portFlag, "port", 443, "Port to check")
	flag.IntVar(&portFlag, "p", 443, "Port to check")
	flag.StringVar(&outputFormatFlag, "format", "short", "Format the output")
	flag.StringVar(&outputFormatFlag, "f", "short", "Format the output")
	flag.BoolVar(&insecureFlag, "insecure", false, "Whether to bypass secure flag checks")
	flag.BoolVar(&insecureFlag, "i", false, "Whether to bypass secure flag checks")
	flag.BoolVar(&versionFlag, "version", false, "Print version information")
	flag.BoolVar(&versionFlag, "v", false, "Print version information")

	flag.Parse()

	if versionFlag {
		if Version != "" {
			fmt.Printf("version: %s, build on: %s\n", Version, BuildDate)
			return
		}
		fmt.Println("(unknown)")
		return
	}

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
		fmt.Printf("%s, valid until %s (%d days left)\n", cert.Subject.CommonName, cert.NotAfter.Format(time.RFC1123), int(remainingDays.Hours()/24))
	} else {
		fmt.Printf("%s, not valid since %s (expired %d days ago)\n", cert.Subject.CommonName, cert.NotAfter.Format(time.RFC1123), int(remainingDays.Abs().Hours()/24))
	}
}

const tmplLong = `certificate
  version: {{ .Version }}
  serial: {{ .SerialNumber }}
  subject: {{ .Subject.CommonName }}
  issuer: {{ .Issuer.CommonName }}

validity:
  not before: {{ rfc1123 .NotBefore }}
  not after: {{ rfc1123 .NotAfter }}
  validity days: {{ validFor .NotBefore .NotAfter }}
  remaining days: {{ remainingDays .NotAfter }}

SANs:
{{- range $i, $name := .DNSNames }}
  • {{ $name }}
{{- end }}
`

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
		"rfc1123": func(date time.Time) string {
			return date.Format(time.RFC1123)
		},
	}

	tmpl, err := template.New("tmpl").Funcs(funcMap).Parse(tmplLong)
	if err != nil {
		panic(err)
	}

	err = tmpl.Execute(os.Stdout, certs[0])
	if err != nil {
		panic(err)
	}
}
