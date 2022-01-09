import time
import picamera
import numpy as np
import cv2

class Camera:
    def __init__(self):
        self.camera = picamera.PiCamera()
        self.resolution.x = 2592
        self.resolution.y = 1944
        self.camera.framerate = 24

    def capture_opencv(self):
        image = np.empty((self.resolution.y * self.resolution.x * 3,), dtype=np.uint8)
        self.camera.capture(image, 'bgr')
        image = image.reshape((self.resolution.y, self.resolution.x, 3))
        return image