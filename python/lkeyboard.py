import keyboard
import time
from pythonosc import udp_client

while True:
    some_key = keyboard.read_key()
    print("oh key!", some_key)

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

keys = ['a', 's', 'd', 'f']

all_keys = keys + ['+', '-']

pressed_keys = []

current_key = ''

def key_pressed(key):
    global current_key
    if key in keys:
        current_key = key
        send(key)
        print("Key played", key)
    elif key in ['+', '-'] and current_key:
        amount = -1 if key == '-' else 1
        mod(current_key, amount)
        print("Mod executed!")       
while True:   

    for key in all_keys:
        if keyboard.is_pressed(key) and not (key in pressed_keys):
            pressed_keys.append(key)
            key_pressed(key)
            time.sleep(0.2)
        elif key in pressed_keys:
            print("Key released", key)
            pressed_keys.remove(key)

  
