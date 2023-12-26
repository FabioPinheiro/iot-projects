

arduino-cli compile -b esp8266:esp8266:generic  ../automatic_blinds
arduino-cli upload -p /dev/ttyUSB0 -b esp8266:esp8266:generic ../automatic_blinds -v
