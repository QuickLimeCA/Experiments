# Experiments

This repository contains code experiments for exploring ideas for the QuickLime project.

## x509parse

This experiment is to explore the openssl crate and gain experience with extracting details from X.509 certificates [RFC5280](https://datatracker.ietf.org/doc/rfc5280/).

This experiment uses the rust-openssl crate, which requires the OpenSSL library and headers installed to build. Follow the instructions on the project page to get started [https://github.com/sfackler/rust-openssl](https://github.com/sfackler/rust-openssl).

x509parse works with PEM formatted certificates. To easily get a certificate that x509parse can work with, run the following command, substituting in the domain you want to download a certificate for.

`openssl s_client -connect google.com:443 -showcerts`

You will see what could potentially be a quite long output, containing lots of information from that TLS handshake.
Copy the 
Example:

```
[dan@localhost x509parse]$ openssl s_client -connect google.com:443 -showcerts
CONNECTED(00000003)
depth=2 C = US, O = GeoTrust Inc., CN = GeoTrust Global CA
verify return:1
depth=1 C = US, O = Google Inc, CN = Google Internet Authority G2
verify return:1
depth=0 C = US, ST = California, L = Mountain View, O = Google Inc, CN = *.google.com
verify return:1
---
Certificate chain
 0 s:/C=US/ST=California/L=Mountain View/O=Google Inc/CN=*.google.com
   i:/C=US/O=Google Inc/CN=Google Internet Authority G2
-----BEGIN CERTIFICATE-----
MIIISDCCBzCgAwIBAgIIdbdIJBtn9J0wDQYJKoZIhvcNAQELBQAwSTELMAkGA1UE
BhMCVVMxEzARBgNVBAoTCkdvb2dsZSBJbmMxJTAjBgNVBAMTHEdvb2dsZSBJbnRl
cm5ldCBBdXRob3JpdHkgRzIwHhcNMTcxMDE3MTAyNjI2WhcNMTcxMjI5MDAwMDAw
WjBmMQswCQYDVQQGEwJVUzETMBEGA1UECAwKQ2FsaWZvcm5pYTEWMBQGA1UEBwwN
TW91bnRhaW4gVmlldzETMBEGA1UECgwKR29vZ2xlIEluYzEVMBMGA1UEAwwMKi5n
b29nbGUuY29tMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAxjuvEeno
7u9q/zXsregzV0p+H/V7rXkgYoHzmiMabTsC93Ci1iwzK5/wR5Pbk4/rasfgGpeo
JlhexewsussgWLd/KXKKUw66vicHY80Por4jESLYzROzALWsJ3ThzIPk3KFpxxGg
V+TjZZd2TLQSbTqt6lcTvFIyVIZxvRgbHZoDU5dcCCpKX3CVMqWPjQoMEafACS0/
CQyRfUH4dZX00+52muSPgUs+odIpAMoS8HQJOFWuT6GYwc68CTloNn4QxPE1yc8D
Zr+WfWWTEurWCT64wX3IUqSq0NpG3kMviMB5RLpm2r7XdaaCYmin2FypMnrYJ8IP
15bkeQPEgBzO6QIDAQABo4IFFTCCBREwHQYDVR0lBBYwFAYIKwYBBQUHAwEGCCsG
AQUFBwMCMIID4QYDVR0RBIID2DCCA9SCDCouZ29vZ2xlLmNvbYINKi5hbmRyb2lk
LmNvbYIWKi5hcHBlbmdpbmUuZ29vZ2xlLmNvbYISKi5jbG91ZC5nb29nbGUuY29t
ghQqLmRiODMzOTUzLmdvb2dsZS5jboIGKi5nLmNvgg4qLmdjcC5ndnQyLmNvbYIW
Ki5nb29nbGUtYW5hbHl0aWNzLmNvbYILKi5nb29nbGUuY2GCCyouZ29vZ2xlLmNs
gg4qLmdvb2dsZS5jby5pboIOKi5nb29nbGUuY28uanCCDiouZ29vZ2xlLmNvLnVr
gg8qLmdvb2dsZS5jb20uYXKCDyouZ29vZ2xlLmNvbS5hdYIPKi5nb29nbGUuY29t
LmJygg8qLmdvb2dsZS5jb20uY2+CDyouZ29vZ2xlLmNvbS5teIIPKi5nb29nbGUu
Y29tLnRygg8qLmdvb2dsZS5jb20udm6CCyouZ29vZ2xlLmRlggsqLmdvb2dsZS5l
c4ILKi5nb29nbGUuZnKCCyouZ29vZ2xlLmh1ggsqLmdvb2dsZS5pdIILKi5nb29n
bGUubmyCCyouZ29vZ2xlLnBsggsqLmdvb2dsZS5wdIISKi5nb29nbGVhZGFwaXMu
Y29tgg8qLmdvb2dsZWFwaXMuY26CFCouZ29vZ2xlY29tbWVyY2UuY29tghEqLmdv
b2dsZXZpZGVvLmNvbYIMKi5nc3RhdGljLmNugg0qLmdzdGF0aWMuY29tggoqLmd2
dDEuY29tggoqLmd2dDIuY29tghQqLm1ldHJpYy5nc3RhdGljLmNvbYIMKi51cmNo
aW4uY29tghAqLnVybC5nb29nbGUuY29tghYqLnlvdXR1YmUtbm9jb29raWUuY29t
gg0qLnlvdXR1YmUuY29tghYqLnlvdXR1YmVlZHVjYXRpb24uY29tggcqLnl0LmJl
ggsqLnl0aW1nLmNvbYIaYW5kcm9pZC5jbGllbnRzLmdvb2dsZS5jb22CC2FuZHJv
aWQuY29tghtkZXZlbG9wZXIuYW5kcm9pZC5nb29nbGUuY26CHGRldmVsb3BlcnMu
YW5kcm9pZC5nb29nbGUuY26CBGcuY2+CBmdvby5nbIIUZ29vZ2xlLWFuYWx5dGlj
cy5jb22CCmdvb2dsZS5jb22CEmdvb2dsZWNvbW1lcmNlLmNvbYIYc291cmNlLmFu
ZHJvaWQuZ29vZ2xlLmNuggp1cmNoaW4uY29tggp3d3cuZ29vLmdsggh5b3V0dS5i
ZYILeW91dHViZS5jb22CFHlvdXR1YmVlZHVjYXRpb24uY29tggV5dC5iZTBoBggr
BgEFBQcBAQRcMFowKwYIKwYBBQUHMAKGH2h0dHA6Ly9wa2kuZ29vZ2xlLmNvbS9H
SUFHMi5jcnQwKwYIKwYBBQUHMAGGH2h0dHA6Ly9jbGllbnRzMS5nb29nbGUuY29t
L29jc3AwHQYDVR0OBBYEFEBHm4uRfnoAZUppM9mNwo+a5o1TMAwGA1UdEwEB/wQC
MAAwHwYDVR0jBBgwFoAUSt0GFhu89mi1dvWBtrtiGrpagS8wIQYDVR0gBBowGDAM
BgorBgEEAdZ5AgUBMAgGBmeBDAECAjAwBgNVHR8EKTAnMCWgI6Ahhh9odHRwOi8v
cGtpLmdvb2dsZS5jb20vR0lBRzIuY3JsMA0GCSqGSIb3DQEBCwUAA4IBAQBMS3jq
FF2oqqUx1byq0hls1FrFgc+3ft9LkkAI/8kYpkAjerL9+Y4w7yV5GIiIIXPEfYXU
ZcSF6GEu8cg1GykNqRSXWKveI+mDGsKlUss7qDj65lJb4CS6KDXcIiQG9KlnIXL6
Ya8lbkIyJYo1SO9ayXkec9wYkYi6FgbcIhI2Ba2APrbrZ95p0BS5a6P8NzlKQoAl
wljX8D6LwjJEyQStdj49XZFE+UPmdXETB1Q1FJlkn6Onzp4UhaW30LwUguK3HuOX
2oBE0nuDBBnI1FtjmElT2hdFRkqrNUc3dI0hTE7JTLAiYgSYwsx0VcKtKxxcaulH
v7EiDAugu2Qpds+7
-----END CERTIFICATE-----
 1 s:/C=US/O=Google Inc/CN=Google Internet Authority G2
   i:/C=US/O=GeoTrust Inc./CN=GeoTrust Global CA
-----BEGIN CERTIFICATE-----
MIIEKDCCAxCgAwIBAgIQAQAhJYiw+lmnd+8Fe2Yn3zANBgkqhkiG9w0BAQsFADBC
MQswCQYDVQQGEwJVUzEWMBQGA1UEChMNR2VvVHJ1c3QgSW5jLjEbMBkGA1UEAxMS
R2VvVHJ1c3QgR2xvYmFsIENBMB4XDTE3MDUyMjExMzIzN1oXDTE4MTIzMTIzNTk1
OVowSTELMAkGA1UEBhMCVVMxEzARBgNVBAoTCkdvb2dsZSBJbmMxJTAjBgNVBAMT
HEdvb2dsZSBJbnRlcm5ldCBBdXRob3JpdHkgRzIwggEiMA0GCSqGSIb3DQEBAQUA
A4IBDwAwggEKAoIBAQCcKgR3XNhQkToGo4Lg2FBIvIk/8RlwGohGfuCPxfGJziHu
Wv5hDbcyRImgdAtTT1WkzoJile7rWV/G4QWAEsRelD+8W0g49FP3JOb7kekVxM/0
Uw30SvyfVN59vqBrb4fA0FAfKDADQNoIc1Fsf/86PKc3Bo69SxEE630k3ub5/DFx
+5TVYPMuSq9C0svqxGoassxT3RVLix/IGWEfzZ2oPmMrhDVpZYTIGcVGIvhTlb7j
gEoQxirsupcgEcc5mRAEoPBhepUljE5SdeK27QjKFPzOImqzTs9GA5eXA37Asd57
r0Uzz7o+cbfe9CUlwg01iZ2d+w4ReYkeN8WvjnJpAgMBAAGjggERMIIBDTAfBgNV
HSMEGDAWgBTAephojYn7qwVkDBF9qn1luMrMTjAdBgNVHQ4EFgQUSt0GFhu89mi1
dvWBtrtiGrpagS8wDgYDVR0PAQH/BAQDAgEGMC4GCCsGAQUFBwEBBCIwIDAeBggr
BgEFBQcwAYYSaHR0cDovL2cuc3ltY2QuY29tMBIGA1UdEwEB/wQIMAYBAf8CAQAw
NQYDVR0fBC4wLDAqoCigJoYkaHR0cDovL2cuc3ltY2IuY29tL2NybHMvZ3RnbG9i
YWwuY3JsMCEGA1UdIAQaMBgwDAYKKwYBBAHWeQIFATAIBgZngQwBAgIwHQYDVR0l
BBYwFAYIKwYBBQUHAwEGCCsGAQUFBwMCMA0GCSqGSIb3DQEBCwUAA4IBAQDKSeWs
12Rkd1u+cfrP9B4jx5ppY1Rf60zWGSgjZGaOHMeHgGRfBIsmr5jfCnC8vBk97nsz
qX+99AXUcLsFJnnqmseYuQcZZTTMPOk/xQH6bwx+23pwXEz+LQDwyr4tjrSogPsB
E4jLnD/lu3fKOmc2887VJwJyQ6C9bgLxRwVxPgFZ6RGeGvOED4Cmong1L7bHon8X
fOGLVq7uZ4hRJzBgpWJSwzfVO+qFKgE4h6LPcK2kesnE58rF2rwjMvL+GMJ74N87
L9TQEOaWTPtEtyFkDbkAlDASJodYmDkFOA/MgkgMCkdm7r+0X8T/cKjhf4t5K7hl
MqO5tzHpCvX2HzLc
-----END CERTIFICATE-----
 2 s:/C=US/O=GeoTrust Inc./CN=GeoTrust Global CA
   i:/C=US/O=Equifax/OU=Equifax Secure Certificate Authority
-----BEGIN CERTIFICATE-----
MIIDfTCCAuagAwIBAgIDErvmMA0GCSqGSIb3DQEBBQUAME4xCzAJBgNVBAYTAlVT
MRAwDgYDVQQKEwdFcXVpZmF4MS0wKwYDVQQLEyRFcXVpZmF4IFNlY3VyZSBDZXJ0
aWZpY2F0ZSBBdXRob3JpdHkwHhcNMDIwNTIxMDQwMDAwWhcNMTgwODIxMDQwMDAw
WjBCMQswCQYDVQQGEwJVUzEWMBQGA1UEChMNR2VvVHJ1c3QgSW5jLjEbMBkGA1UE
AxMSR2VvVHJ1c3QgR2xvYmFsIENBMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIB
CgKCAQEA2swYYzD99BcjGlZ+W988bDjkcbd4kdS8odhM+KhDtgPpTSEHCIjaWC9m
OSm9BXiLnTjoBbdqfnGk5sRgprDvgOSJKA+eJdbtg/OtppHHmMlCGDUUna2YRpIu
T8rxh0PBFpVXLVDviS2Aelet8u5fa9IAjbkU+BQVNdnARqN7csiRv8lVK83Qlz6c
JmTM386DGXHKTubU1XupGc1V3sjs0l44U+VcT4wt/lAjNvxm5suOpDkZALeVAjmR
Cw7+OC7RHQWa9k0+bw8HHa8sHo9gOeL6NlMTOdReJivbPagUvTLrGAMoUgRx5asz
PeE4uwc2hGKceeoWMPRfwCvocWvk+QIDAQABo4HwMIHtMB8GA1UdIwQYMBaAFEjm
aPkr0rKV10fYIyAQTzOYkJ/UMB0GA1UdDgQWBBTAephojYn7qwVkDBF9qn1luMrM
TjAPBgNVHRMBAf8EBTADAQH/MA4GA1UdDwEB/wQEAwIBBjA6BgNVHR8EMzAxMC+g
LaArhilodHRwOi8vY3JsLmdlb3RydXN0LmNvbS9jcmxzL3NlY3VyZWNhLmNybDBO
BgNVHSAERzBFMEMGBFUdIAAwOzA5BggrBgEFBQcCARYtaHR0cHM6Ly93d3cuZ2Vv
dHJ1c3QuY29tL3Jlc291cmNlcy9yZXBvc2l0b3J5MA0GCSqGSIb3DQEBBQUAA4GB
AHbhEm5OSxYShjAGsoEIz/AIx8dxfmbuwu3UOx//8PDITtZDOLC5MH0Y0FWDomrL
NhGc6Ehmo21/uBPUR/6LWlxz/K7ZGzIZOKuXNBSqltLroxwUCEm2u+WR74M26x1W
b8ravHNjkOR/ez4iyz0H7V84dJzjA1BOoa+Y7mHyhD8S
-----END CERTIFICATE-----
---
Server certificate
subject=/C=US/ST=California/L=Mountain View/O=Google Inc/CN=*.google.com
issuer=/C=US/O=Google Inc/CN=Google Internet Authority G2
---
No client certificate CA names sent
Peer signing digest: SHA256
Server Temp Key: X25519, 253 bits
---
SSL handshake has read 4768 bytes and written 339 bytes
Verification: OK
---
New, TLSv1.2, Cipher is ECDHE-RSA-CHACHA20-POLY1305
Server public key is 2048 bit
Secure Renegotiation IS supported
Compression: NONE
Expansion: NONE
No ALPN negotiated
SSL-Session:
    Protocol  : TLSv1.2
    Cipher    : ECDHE-RSA-CHACHA20-POLY1305
    Session-ID: EC847585EAC8C03406D21AF5E2FF69249FB6D6A4F5E96CD36DFCDBF0D4E6183A
    Session-ID-ctx: 
    Master-Key: 5226EF6F97A3A33B879867C7C89200FBCE35AE5DD1AF4EF9758CA31E210B148B3705E4AF62F018A9C2F9B45B99781A43
    PSK identity: None
    PSK identity hint: None
    SRP username: None
    TLS session ticket lifetime hint: 100800 (seconds)
    TLS session ticket:
    0000 - 00 18 8f 07 fa 02 4e bd-2e f9 e6 71 93 4e 1a 79   ......N....q.N.y
    0010 - 27 88 5e 4e e2 35 60 6a-31 06 71 d3 68 1f 13 78   '.^N.5`j1.q.h..x
    0020 - ad cf d7 ed b9 31 28 1a-33 1f 20 15 46 cf 65 75   .....1(.3. .F.eu
    0030 - b7 53 c3 4f ce ed d0 01-86 43 cd ba 2c 9b 83 3a   .S.O.....C..,..:
    0040 - 9d 9b 90 8f 03 cb c7 b7-f2 52 ad a2 ce 2e 14 26   .........R.....&
    0050 - 33 22 e0 98 8f df dd 69-6a f7 c4 f6 2f 4b d7 d2   3".....ij.../K..
    0060 - b4 94 4b 5c 90 61 ad 8e-9d 8c be 5d 89 93 ef 0a   ..K\.a.....]....
    0070 - 28 c4 64 a2 23 56 67 89-98 39 63 00 46 e8 d2 f4   (.d.#Vg..9c.F...
    0080 - ed e9 aa 46 aa 26 ec ef-cb ee c4 79 2d d5 b5 c6   ...F.&.....y-...
    0090 - 92 f3 b6 d6 64 ca 9f 2d-a1 0d dc 28 23 f1 0d a5   ....d..-...(#...
    00a0 - 4e 4e 29 54 24 37 14 51-46 20 b3 f1 f1 f1 c5 ab   NN)T$7.QF ......
    00b0 - 5a af d5 4a 92 9b b7 7c-d1 29 cb 8c 69 f6 73 6d   Z..J...|.)..i.sm
    00c0 - 29 a0 1b c0 cf b6 fc 65-ec 52 e4 43 1b 47 44 ad   )......e.R.C.GD.
    00d0 - a7 52 e6 8d 25 f6 13 f0-64                        .R..%...d
    00da - <SPACES/NULS>

    Start Time: 1509289691
    Timeout   : 7200 (sec)
    Verify return code: 0 (ok)
    Extended master secret: yes
---
```

Copy the first certificate in the output, including the `-----BEGIN CERTIFICATE-----` and `-----END CERTIFICATE-----` lines and save to a file called testcert.pem

Build the crate using `cargo build --release`

Then, once you've built the crate, run `./target/release/x509parse --file testcert.pem`

## schema

This experiment is to try out the Valico library for [JSON Schema](http://json-schema.org/) validation. I have used a fork of [Valico](https://github.com/korczis/valico/tree/serde-1.0) by Github user [korczis](https://github.com/korczis) in order to gain support for Serde 1.0.

The data folder contains two files; a simple JSON schema (draft-07) and a self-describing JSON file, which is described in a blog post from [Snowplow Analytics](https://snowplowanalytics.com/blog/2014/05/15/introducing-self-describing-jsons/).

Build the crate using `cargo build --release`

Then, once you've built the crate, run `./target/release/schema --json schema/data/example_data.json --schema schema/data/example.json`. You should see `Is JSON Valid? true`.

If you edit the JSON file and remove a property or change the type of testProp from a string to a number, you will see corresponding validation errors in the console.

## structver

This experiment is to figure out an elegant way of handling breaking changes to the schema of a JSON object. This felt like a fundamental problem that would surface when building an Event Sourcing architecture, because the consuming applications need to be able to handle every version of each event in the system in order to avoid data loss.

The solution to this problem is to use Serde's untagged enum deserialisation functionality to deserialise JSON to a variant of an enum, where each variant is a struct. The match expressions include a catch-all to handle unknown versions, but this could be removed if a broken build is the desired behaviour on updating the crate containing the deserialisation target structs.

Build the crate using `cargo build --release`, then run `./target/release/structver`.