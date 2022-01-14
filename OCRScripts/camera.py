import time
camera_lib_loaded = None
try:
    import picamera
    camera_lib_loaded = True
except ModuleNotFoundError as err:
    camera_lib_loaded = False

import numpy as np
import cv2
p
class Camera:
    def __init__(self):
        if camera_lib_loaded == True:
            self.camera = picamera.PiCamera()
            self.camera.framerate = 24
            self.camera.resolution.x = 2592
            self.camera.resolution.y = 1944
            self.camera.rotation = 180
        self.loaded = camera_lib_loaded

    def __del__(self):
        if camera_lib_loaded == True:
            self.camera.close()

    def capture_opencv(self):
        image = np.empty((self.resolution.y * self.resolution.x * 3,), dtype=np.uint8)
        self.camera.capture(image, 'bgr')
        image = image.reshape((self.resolution.y, self.resolution.x, 3))
        return image
