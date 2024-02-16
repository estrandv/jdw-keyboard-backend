import time
from pythonosc import udp_client

# Hardcoded default port of jdw-sc main application
client = udp_client.SimpleUDPClient("127.0.0.1", 13458) # Straight to main application

client.send_message("/modify_letter", [

    'a', 1
])

