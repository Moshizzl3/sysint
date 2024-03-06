resource "azurerm_resource_group" "resource_group" {
  name     = var.ressource_group_name
  location = var.location
}

resource "azurerm_postgresql_flexible_server" "pgserver" {
  name                   = var.postgresql_flexible_server
  resource_group_name    = azurerm_resource_group.resource_group.name
  location               = azurerm_resource_group.resource_group.location
  version                = var.postgres_version
  administrator_login    = var.postgres_user
  administrator_password = var.postgres_password
  zone                   = var.db_zone

  storage_mb = 32768
  sku_name   = "B_Standard_B1ms"
}

resource "azurerm_postgresql_flexible_server_database" "pgdb" {
  name      = "testdb"
  server_id = azurerm_postgresql_flexible_server.pgserver.id
  collation = "en_US.utf8"
  charset   = "utf8"

  lifecycle {
    prevent_destroy = false
  }
}

resource "null_resource" "db_init" {
  depends_on = [azurerm_postgresql_flexible_server.pgserver]

  provisioner "local-exec" {
    environment = {
      PGPASSWORD = var.postgres_password
    }
    command = "psql -h ${azurerm_postgresql_flexible_server.pgserver.name}.postgres.database.azure.com -U ${azurerm_postgresql_flexible_server.pgserver.administrator_login} -d ${azurerm_postgresql_flexible_server_database.pgdb.name} -f init.sql"
  }
}

resource "azurerm_postgresql_flexible_server_firewall_rule" "allow_all" {
  name             = "AllowAll"
  server_id        = azurerm_postgresql_flexible_server.pgserver.id
  start_ip_address = "0.0.0.0"
  end_ip_address   = "255.255.255.255"
}
