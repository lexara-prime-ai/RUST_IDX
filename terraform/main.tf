provider "github" {
  token = var.github_token
  owner = var.github_owner
}

resource "github_repository" "RUST_IDX" {
  name        = "RUST_IDX"
  description = "A sample IDX template."
  visibility  = "public"
}
