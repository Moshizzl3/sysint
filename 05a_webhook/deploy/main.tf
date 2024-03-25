resource "azurerm_resource_group" "webhook" {
  name     = "webhook-resources"
  location = "West Europe"
}

resource "azurerm_service_plan" "webhook_plan" {
  name                = "webhookplan"
  resource_group_name = azurerm_resource_group.webhook.name
  location            = azurerm_resource_group.webhook.location
  os_type             = "Linux"
  sku_name            = "B1"
}

resource "azurerm_linux_web_app" "webhook_app" {
  name                = "webhookapp420"
  resource_group_name = azurerm_resource_group.webhook.name
  location            = azurerm_service_plan.webhook_plan.location
  service_plan_id     = azurerm_service_plan.webhook_plan.id

  site_config {

    always_on = false

    application_stack {
      docker_image_name   = "moshizzl3/webhook:latest"
      docker_registry_url = "https://index.docker.io"
    }
  }
}
