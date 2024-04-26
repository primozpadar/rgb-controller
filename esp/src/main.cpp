#include <ESP8266WiFi.h>
#include <WiFiUdp.h>

#define R 14
#define G 16
#define B 12


//------------ Config -----------//
// #define DEBUG
const char *ssid = "";
const char *password = "";
const char *deviceName = "";

#define MANUAL_NET_CONFIG
#ifdef MANUAL_NET_CONFIG
IPAddress deviceIp(10, 101, 0, 2);
IPAddress gateway(10, 0, 0, 1);
IPAddress subnet(255, 0, 0, 0);
#endif
//-------------------------------//

WiFiUDP UDP;
unsigned int UDP_PORT = 50000;

void displaySuccessfulConnection() {
  digitalWrite(R, LOW);
  for(int i = 0; i <= 4; i++) {
    analogWrite(G, 5);
    delay(200);
    analogWrite(G, 0);
    delay(200);
  }
}

void writeRgb(int r, int g, int b) {
  analogWrite(R, r);
  analogWrite(G, g);
  analogWrite(B, b);
}

void setup() {
  Serial.begin(115200);

  pinMode(R, OUTPUT);
  pinMode(G, OUTPUT);
  pinMode(B, OUTPUT);
  writeRgb(5, 0, 0);

  Serial.printf("Connecting to WIFI: %s\n", ssid);
  WiFi.disconnect();
  WiFi.hostname(deviceName);
  WiFi.mode(WIFI_STA);
  WiFi.setAutoReconnect(true);
  
  #ifdef MANUAL_NET_CONFIG
    WiFi.config(deviceIp, gateway, subnet);
  #endif

  WiFi.begin(ssid, password);
  while (WiFi.status() != WL_CONNECTED) {
    Serial.print(".");
    delay(250);
  }

  Serial.println("\n\nConnected!");

  Serial.print("\nDevice IP: ");
  Serial.print(WiFi.localIP());
  Serial.print("\nGateway: ");
  Serial.print(WiFi.gatewayIP());
  Serial.print("\nSubnet: ");
  Serial.print(WiFi.subnetMask());
  Serial.println();

  displaySuccessfulConnection();

  UDP.begin(UDP_PORT);
}

// BYTES LAYOUT Group-R-G-B
#define MESSAGE_SIZE 4
byte incomingBuffer[MESSAGE_SIZE];

void loop() {
  if (UDP.parsePacket() != MESSAGE_SIZE) {
    return;
  }

  UDP.read(incomingBuffer, MESSAGE_SIZE);

  #ifdef DEBUG
    Serial.printf("Received Message on channel %d (R %d, G %d, B %d)\n", incomingBuffer[0], incomingBuffer[1], incomingBuffer[2], incomingBuffer[3]);
  #endif

  // channels 1 and 3
  if (incomingBuffer[0] == 1 || incomingBuffer[0] == 3) {
    writeRgb(incomingBuffer[1], incomingBuffer[2], incomingBuffer[3]);
  }
}

