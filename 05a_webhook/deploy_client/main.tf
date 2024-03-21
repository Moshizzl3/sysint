resource "azurerm_resource_group" "webhookclient" {
  name     = "webhookclient-resources"
  location = "West Europe"
}

resource "azurerm_service_plan" "webhookclient_plan" {
  name                = "webhookclient_plan"
  resource_group_name = azurerm_resource_group.webhookclient.name
  location            = azurerm_resource_group.webhookclient.location
  os_type             = "Linux"
  sku_name            = "F1"
}

resource "azurerm_linux_web_app" "webhook_app" {
  name                = "webhookapp42069"
  resource_group_name = azurerm_resource_group.webhookclient.name
  location            = azurerm_service_plan.webhookclient_plan.location
  service_plan_id     = azurerm_service_plan.webhookclient_plan.id

  site_config {

    always_on = false

    application_stack {
      docker_image_name   = "moshizzl3/webhookclient:latest"
      docker_registry_url = "https://index.docker.io"
    }
  }
}
