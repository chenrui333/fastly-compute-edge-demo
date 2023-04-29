terraform {
  required_version = "= 1.1.7"

  required_providers {
    fastly = {
      source  = "fastly/fastly"
      version = ">= 1"
    }
  }
}
