terraform {
  required_version = "1.7.5"

  required_providers {
    fastly = {
      source  = "fastly/fastly"
      version = ">= 1"
    }
  }
}
