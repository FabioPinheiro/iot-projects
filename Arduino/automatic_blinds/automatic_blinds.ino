#include <ESP8266WiFi.h>
#include secrets.h

const char *ssid = SECRET_SSID;     // type ssid
const char *password = SECRET_PASS; // type password

// Pins
const int relay1Pin = 2; // GPIO2 (D4) of ESP8266 ESP-01
const int relay2Pin = 0; // GPIO0 (D3) of ESP8266 ESP-01

WiFiServer server(80);

void setup() {
  pinMode(relay1Pin, OUTPUT);
  digitalWrite(relay1Pin, LOW);

  pinMode(relay2Pin, OUTPUT);
  digitalWrite(relay2Pin, LOW);

  Serial.begin(115200);
  delay(10);

  WiFi.begin(ssid, password);

  while (WiFi.status() != WL_CONNECTED) {
    delay(500);
  }

  server.begin();
}

void reply1(bool x) {
  if (x == true) {
    // digitalWrite(relay1Pin, HIGH);
    // Relay 1 On
    Serial.write(0xA0);
    Serial.write(0x01); // Relay 1
    Serial.write(0x01); // On
    Serial.write(0xA2);
    delay(100);
    Serial.flush();
  } else {
    // digitalWrite(relay1Pin, LOW);
    // Relay 1 Off
    Serial.write(0xA0);
    Serial.write(0x01); // Relay 1
    Serial.write(0x00); // Off
    Serial.write(0xA1);
    delay(100);
    Serial.flush();
  }
}

void reply2(bool x) {
  if (x == true) {
    // digitalWrite(relay2Pin, HIGH);
    // Relay 2 On
    Serial.write(0xA0);
    Serial.write(0x02); // Relay 2
    Serial.write(0x01); // On
    Serial.write(0xA3);
    delay(100);
    Serial.flush();
  } else {
    // digitalWrite(relay2Pin, LOW);
    // Relay 2 off
    Serial.write(0xA0);
    Serial.write(0x02); // Relay 2
    Serial.write(0x00); // Off
    Serial.write(0xA2);
    delay(100);
    Serial.flush();
  }
}

void loop() {
  WiFiClient client = server.available();

  if (!client) {
    return;
  }

  while (!client.available()) {
    delay(1);
  }

  String req = client.readStringUntil('\r');
  client.flush();

  if (req.indexOf("/version") != -1) {
    client.println("HTTP/1.1 200 OK");
    client.println("Content-Type: text/html");
    client.println();
    client.println("<h1>Version 6</h1>");

  } else if (req.indexOf("/off") != -1) {
    reply1(false);
    reply2(false);

    client.println("HTTP/1.1 200 OK");
    client.println("Content-Type: text/html");
    client.println();
    client.println("<h1>Relay 1 and Relay 2 are OFF</h1>");
  } else if (req.indexOf("/up") != -1) {
    reply2(false);
    reply1(true);
    delay(1000);
    reply1(false);
    reply2(false);

    client.println("HTTP/1.1 200 OK");
    client.println("Content-Type: text/html");
    client.println();
    client.println("<h1>Relay 1 is ON, Relay 2 is OFF</h1>");
  } else if (req.indexOf("/down") != -1) {
    reply2(true);
    reply1(true);
    delay(1000);
    reply1(false);
    reply2(false);

    client.println("HTTP/1.1 200 OK");
    client.println("Content-Type: text/html");
    client.println();
    client.println("<h1>Relay 1 and Relay 2 are ON</h1>");
  } else {
    reply1(false);
    reply2(false);

    client.println("HTTP/1.1 404 Not Found");
    client.println("Content-Type: text/html");
    client.println();
    client.println("<h1>Not Found</h1>");
  }

  delay(1);
  Serial.println("Client disconnected");
}
