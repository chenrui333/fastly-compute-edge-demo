terraform {
  required_version = "1.5.4"

  required_providers {
    fastly = {
      source  = "fastly/fastly"
      version = ">= 1"
    }
  }
}
