import keyboard
import time
from pythonosc import udp_client

# Hardcoded default port of jdw-sc main application
client = udp_client.SimpleUDPClient("127.0.0.1", 13458) # Straight to main application

def send(letter):
    client.send_message("/trigger_letter", [
        letter
    ])

def mod(letter, amount):
    client.send_message("/modify_letter", [
        letter, amount
    ])

while True:  # making a loop
    if keyboard.is_pressed('a'):  # if key 'q' is pressed
        send('a')
        print("A!")
