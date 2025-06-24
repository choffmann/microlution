#include "esp_camera.h"
#include <WiFi.h>
#include <WebServer.h>

const char* ssid = "Microlution";
const char* password = "knorke123";

WiFiClient streamClient;
bool isStreaming = false;

// Set camera model (AI Thinker)
#define PWDN_GPIO_NUM     32
#define RESET_GPIO_NUM    -1
#define XCLK_GPIO_NUM      0
#define SIOD_GPIO_NUM     26
#define SIOC_GPIO_NUM     27

#define Y9_GPIO_NUM       35
#define Y8_GPIO_NUM       34
#define Y7_GPIO_NUM       39
#define Y6_GPIO_NUM       36
#define Y5_GPIO_NUM       21
#define Y4_GPIO_NUM       19
#define Y3_GPIO_NUM       18
#define Y2_GPIO_NUM        5
#define VSYNC_GPIO_NUM    25
#define HREF_GPIO_NUM     23
#define PCLK_GPIO_NUM     22

WebServer server(80);

void startCamera() {
  camera_config_t config;
  config.ledc_channel = LEDC_CHANNEL_0;
  config.ledc_timer = LEDC_TIMER_0;
  config.pin_d0 = Y2_GPIO_NUM;
  config.pin_d1 = Y3_GPIO_NUM;
  config.pin_d2 = Y4_GPIO_NUM;
  config.pin_d3 = Y5_GPIO_NUM;
  config.pin_d4 = Y6_GPIO_NUM;
  config.pin_d5 = Y7_GPIO_NUM;
  config.pin_d6 = Y8_GPIO_NUM;
  config.pin_d7 = Y9_GPIO_NUM;
  config.pin_xclk = XCLK_GPIO_NUM;
  config.pin_pclk = PCLK_GPIO_NUM;
  config.pin_vsync = VSYNC_GPIO_NUM;
  config.pin_href = HREF_GPIO_NUM;
  config.pin_sscb_sda = SIOD_GPIO_NUM;
  config.pin_sscb_scl = SIOC_GPIO_NUM;
  config.pin_pwdn = PWDN_GPIO_NUM;
  config.pin_reset = RESET_GPIO_NUM;
  config.xclk_freq_hz = 20000000;
  config.pixel_format = PIXFORMAT_JPEG;

  config.frame_size = FRAMESIZE_QVGA;
  config.jpeg_quality = 10;
  config.fb_count = 2;

  esp_err_t err = esp_camera_init(&config);
  if (err != ESP_OK) {
    while (true) delay(1000);
  }
}

void stream() {
  streamClient = server.client();
  isStreaming = true;

  String response = "HTTP/1.1 200 OK\r\n";
  response += "Content-Type: multipart/x-mixed-replace; boundary=frame\r\n\r\n";
  streamClient.print(response);
}

void capture() {
  camera_fb_t *frameBuffer = esp_camera_fb_get();
  if (!frameBuffer) {
    server.send(500, "text/plain", "Camera capture failed");
    return;
  }

  server.setContentLength(frameBuffer->len);
  server.send(200, "image/jpeg", "");

  WiFiClient client = server.client();
  client.write(frameBuffer->buf, frameBuffer->len);

  esp_camera_fb_return(frameBuffer);
}

void setup() {
  Serial.begin(115200);
  delay(1000);

  WiFi.begin(ssid, password);
  while (WiFi.status() != WL_CONNECTED) {
    delay(500);
  }


  startCamera();

  server.on("/stream", HTTP_GET, stream);
  server.on("/capture", HTTP_GET, capture);
  server.begin();
}

void loop() {
  server.handleClient();

  if (isStreaming && streamClient.connected()) {
    camera_fb_t *frameBuffer = esp_camera_fb_get();
    if (frameBuffer) {
      streamClient.write(frameBuffer->buf, frameBuffer->len);
      esp_camera_fb_return(frameBuffer);
    }
    delay(100);
  } else if (isStreaming) {
    isStreaming = false;
  }
}
