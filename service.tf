resource "fastly_service_compute" "fastly_compute_edge_demo" {
  name    = "fastly-compute-edge-demo"
  comment = "Managed by chenrui333/fastly-compute-edge-demo repo"

  domain {
    name = "fastly-compute-edge-demo.edgecompute.app"
  }

  backend {
    address = "127.0.0.1"
    name    = "originless"
    port    = 80
  }

  backend {
    name    = "httpbin"
    address = "httpbin.org"
    port    = 443

    ssl_cert_hostname = "httpbin.org"
    ssl_check_cert    = true
    ssl_sni_hostname  = "httpbin.org"
    use_ssl           = true
    min_tls_version   = "1.2"
  }

  package {
    filename         = "pkg/fastly-compute-edge-demo.tar.gz"
    source_code_hash = filesha512("pkg/fastly-compute-edge-demo.tar.gz")
  }

  force_destroy = true
}
