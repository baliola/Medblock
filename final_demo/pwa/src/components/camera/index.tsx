"use client"

import { fileChecker, imageTypes } from "@/utils/file-checker";
import { Button, Drawer, DrawerBody, DrawerContent, DrawerOverlay, Flex, useToast } from "@chakra-ui/react";
import { useCallback, useRef } from "react";
import Webcam from "react-webcam";

interface CameraInputProps {
  setFile: (file: File | null) => void;
  isOpen: boolean;
  onClose: () => void;
}

export default function CameraInput({ setFile, isOpen, onClose }: CameraInputProps) {
  const toast = useToast();
  const webcamRef = useRef<Webcam>(null);

  const videoConstraints = {
    width: 1280,
    height: 720,
    facingMode: "environment"
  };

  const onCameraNotReady = () => {
    onClose();
    toast.closeAll();
    return toast({
      title: "Can not access your camera",
      description: "Camera is not ready in your device",
      status: "error",
      duration: 5000,
      isClosable: true,
      position: "top-right"
    });

  }

  const capture = useCallback(() => {
    if (webcamRef.current) {
      const imageSrc = webcamRef.current.getScreenshot();
      if (imageSrc) {
        fetch(imageSrc)
          .then((res) => res.blob())
          .then((blob) => {
            const file = new File([blob], 'id-card.jpg', { type: 'image/jpeg' });

            try {
              const checker = fileChecker({
                file,
                maxSize: 1024 * 1024,
                allowedTypes: imageTypes
              });

              if (checker) {
                setFile(file);
                onClose();
              }
            } catch (error) {
              if (error instanceof Error) {
                return toast({
                  title: "File error",
                  description: error.message,
                  status: "error",
                  duration: 5000,
                  isClosable: true,
                  position: "top-right"
                });
              }
            }
          });
      }
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [webcamRef, setFile, onClose]);



  return (
    <Drawer isOpen={isOpen} onClose={onClose} size={"full"}>
      <DrawerOverlay />
      <DrawerContent>
        <DrawerBody>
          <Flex maxW={'1280px'} mx={'auto'} align={'center'} justify={'center'} direction={'column'}>
            <Webcam
              audio={false}
              height={720}
              ref={webcamRef}
              screenshotFormat="image/jpeg"
              width={1280}
              videoConstraints={videoConstraints}
              onUserMediaError={onCameraNotReady}
            />
            <Flex align={'center'} justify={'center'} w={'full'} mt={4} gap={5}>
              <Button
                colorScheme={"primary"}
                bg={"primary.700"}
                w={'full'}
                onClick={capture}>
                Capture
              </Button>
              <Button
                colorScheme={"red"}
                w={'full'}
                onClick={onClose}
              >
                Cancel
              </Button>
            </Flex>
          </Flex>
        </DrawerBody>
      </DrawerContent>
    </Drawer>
  )
}