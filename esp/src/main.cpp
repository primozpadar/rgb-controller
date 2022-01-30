#include <ESP8266WiFi.h>
#include <WiFiUdp.h>

#define R 14
#define G 16
#define B 12

//------------ Config -----------//
const char *ssid = "";
const char *password = "";
const char *deviceName = "RGB-controller";
const char *deviceId = "01";
//-------------------------------//

WiFiUDP UDP;
unsigned int UDP_PORT = 50000;

char incomingBuffer[32];
int size;

void setup() {
  Serial.begin(115200);
  delay(3000);

  pinMode(R, OUTPUT);
  pinMode(G, OUTPUT);
  pinMode(B, OUTPUT);
  digitalWrite(R, HIGH);
  digitalWrite(G, HIGH);
  digitalWrite(B, HIGH);

  Serial.printf("Connecting to %s\n", ssid);
  WiFi.disconnect();
  WiFi.hostname(deviceName);
  WiFi.mode(WIFI_STA);
  WiFi.begin(ssid, password);
  while (WiFi.status() != WL_CONNECTED) {
    Serial.print(".");
    delay(250);
  }
  Serial.println("\nConnected!\n");

  UDP.begin(UDP_PORT);
}

int getColor(char *buffer, int start) {
  char x[] = {buffer[start], buffer[start + 1], buffer[start + 2]};
  return atoi(x);
}

void writeRgb(int r, int g, int b) {
  analogWrite(R, r);
  analogWrite(G, g);
  analogWrite(B, b);
}

bool isDeviceAddress(char *buffer) {
  char incomingAddr[] = {buffer[0], buffer[1]};
  return strcmp(incomingAddr, deviceId) == 0 || strcmp(incomingAddr, "00") == 0;
}

void cmdHandler(char *buffer) {
  if(!isDeviceAddress(buffer)) {
    return;
  }

  switch (buffer[2]) {
  case 'C':
    writeRgb(getColor(buffer, 3), getColor(buffer, 6), getColor(buffer, 9));
    break;
  }
}

void loop() {
  size = UDP.parsePacket();
  if (size) {
    UDP.read(incomingBuffer, size);
    cmdHandler(incomingBuffer);
  }
}
