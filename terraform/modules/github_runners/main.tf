variable "runner_count" { type = number }
variable "instance_type" { type = string }
variable "ami" { default = "ami-0ea3c35c5c3284d82" } # ubuntu-24.04
variable "security_group_id" { type = string }
variable "volume_size" { default = 80 }

resource "aws_instance" "github_runner_server" {
  count = var.runner_count

  ami               = var.ami
  instance_type     = var.instance_type
  vpc_security_group_ids = [var.security_group_id]

  root_block_device {
    volume_size = var.volume_size
  }

  key_name = "aws-infra"
  tags = {
    module = "vlayer-github-runners"
  }
}

output "instance_ips" {
  value = aws_instance.github_runner_server.*.public_ip
}
