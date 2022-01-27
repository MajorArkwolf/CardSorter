from numpy import imag
from MTGFactory import Generate_Magic_Card, Get_Card_From_Bytes, Get_Card_From_File
from MTGLibrary import *
from Network import Response
from camera import Camera
import cv2
from datetime import datetime
import os

file_path = "./card-output/"

camera = Camera()

def from_binary(byte_stream):
    image = Get_Card_From_Bytes(byte_stream)
    return image

async def take_photo():
    if camera.loaded == True:
        return await camera.capture_opencv()
    else:
        raise Exception("Camera module not loaded")

def load_from_file(file_location):
    return Get_Card_From_File(file_location)

def save_card(card):
    dt_string = datetime.now().strftime("%d/%m/%Y %H:%M:%S")
    output_dir = file_path + dt_string
    os.makedirs(output_dir)
    if card.image is not None:
        cv2.imwrite(output_dir + "/image.jpg", card.image)
    file_name = output_dir + "/cardtext.txt"
    with open(file_name, 'w') as f:
        f.write("Card Name:" + card.card_name + '\n')
        f.write("Card Copyright Year:" + card.copyright + '\n')
        f.write("Card Flavor Text:" + card.flavour_text + '\n')

async def card_request(library, request):
    image = None
    if request['type_of']['type'] == "Binary":
        image = from_binary(request["data"])
    elif request['type_of']['type'] == "FileLocation":
        image = load_from_file(request["data"])
    elif request['type_of']['type'] == "TakePicture":
        image = await take_photo()
    else:
        return Response(-2, 0)

    card = Generate_Magic_Card(image)

    save_card(card)
    if card.found == True:
        card_search = library.Look_Up_Card(card)
        return Response(0, 1)
    else:
        return Response(0, 0)