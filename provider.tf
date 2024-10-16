terraform {
  required_version = "1.9.8"

  required_providers {
    fastly = {
      source  = "fastly/fastly"
      version = ">= 1.0.0"
    }
  }
}
