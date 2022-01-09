from MTGFactory import Generate_Magic_Card, Get_Card_From_Bytes, Get_Card_From_File
from MTGLibrary import *
from Network import Response
from camera import Camera

def from_binary( request):
    image = Get_Card_From_Bytes(request["data"])
    return image

def take_photo():
    camera = Camera()
    return camera.capture_opencv()

def load_from_file(file_location):
    return Get_Card_From_File(file_location)

async def card_request(library, request):
    image = None
    if request['CardData'] == "Binary":
        image = from_binary(request)
    elif request['CardData'] == "FileLocation":
        image = load_from_file()
    elif request['CardData'] == "TakePhoto":
        image = take_photo()

    card = Generate_Magic_Card(image)
    card_search = library.Look_Up_Card(card)
    return Response(0, 1)