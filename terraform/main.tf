provider "aws" {
  region = "us-east-2"
  default_tags {
    tags = {
      project = "vlayer"
      owner   = "devops"
    }
  }
}

resource "aws_security_group" "ssh_security_group" {
  egress = [
    {
      cidr_blocks      = ["0.0.0.0/0", ]
      description      = "ALL Outbound"
      from_port        = 0
      ipv6_cidr_blocks = []
      prefix_list_ids  = []
      protocol         = "-1"
      security_groups  = []
      self             = false
      to_port          = 0
    }
  ]
  ingress = [
    {
      cidr_blocks      = ["0.0.0.0/0", ]
      description      = "SSH Inbound"
      from_port        = 22
      ipv6_cidr_blocks = []
      prefix_list_ids  = []
      protocol         = "tcp"
      security_groups  = []
      self             = false
      to_port          = 22
    }
  ]
}

module "github_runners_medium" {
  source            = "./modules/github_runners"
  runner_count      = 2
  instance_type     = "t2.xlarge"
  security_group_id = aws_security_group.ssh_security_group.id
}

module "github_runners_small" {
  source            = "./modules/github_runners"
  runner_count      = 2
  instance_type     = "t2.large"
  security_group_id = aws_security_group.ssh_security_group.id
}

resource "local_file" "github_runners_inventory" {
  content = templatefile("./github-runners.ini.j2",
    {
      github_runners_medium = flatten(module.github_runners_medium.*.instance_ips),
      github_runners_small = flatten(module.github_runners_small.*.instance_ips)
    }
  )
  filename = "./github-runners.ini"
}
