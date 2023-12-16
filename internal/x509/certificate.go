package x509

import (
	"crypto/tls"
	"crypto/x509"
	"fmt"
)

func GetCertificates(domain string, port int, insecureSkipVerify bool) ([]*x509.Certificate, error) {
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
