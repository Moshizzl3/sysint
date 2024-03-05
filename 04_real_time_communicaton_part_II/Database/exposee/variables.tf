variable "location" {
  type        = string
  description = "Azure server location"
  default     = "westeurope"
}

variable "ressource_group_name" {
  type        = string
  description = "Name of the ressource group"
}


variable "postgresql_flexible_server" {
  type        = string
  description = "name of the postgres server"
}

variable "postgres_version" {
  type        = string
  description = "Postgres version number, must be as string"
}

variable "postgres_user" {
  type        = string
  description = "postgres user name"
}

variable "postgres_password" {
  type        = string
  description = "password for postgress"
  sensitive   = false
}
variable "db_zone" {
  type        = string
  description = "Zone to where to distribute it"
}
variable "postgres_ip_rules" {
  description = "Map of PostgreSQL firewall rules"
  default = {
    "AllowAll"      = "0.0.0.0"        // Rule to allow access from all IPs
  }
}
