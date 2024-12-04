"use client"

import { Drawer, DrawerBody, DrawerContent, Flex, FlexProps, FormControl, FormLabel, Image, Input, Text, useDisclosure, useToast } from "@chakra-ui/react";
import { Fragment, useRef } from "react";
import CameraInput from "@/components/camera";
import { fileChecker, imageTypes } from "@/utils/file-checker";
import { useFormikContext } from "formik";
import { PatientRegister } from "@/libs/yup/patients-registration";

// 10MB
const MAX_SIZE = 1024 * 1024 * 10;

interface IDCardSelectionProps {
  isOpen: boolean;
  onClose: () => void;
  setFile: (file: File | null) => void;
}

const SelectionContainer = ({ children, ...props }: FlexProps) => (
  <Flex
    direction={"column"}
    gap={3}
    bg={"white"}
    align={'center'}
    w={'full'}
    p={5}
    rounded={"xl"}
    transition={"all 0.3s"}
    _hover={{ bg: "gray.50" }}
    {...props}
  >
    {children}
  </Flex>
)

const IDCardSelection = ({ isOpen, onClose, setFile }: IDCardSelectionProps) => {
  const toast = useToast();
  const { setFieldValue } = useFormikContext();
  const {
    isOpen: isCameraOpen,
    onOpen: onCameraOpen,
    onClose: onCameraClose
  } = useDisclosure();

  const refUpload = useRef<HTMLInputElement>(null);

  const onUploadFile = (e: React.ChangeEvent<HTMLInputElement>) => {
    const files = e.target.files;
    try {
      if (files && files.length > 0) {
        const checker = fileChecker({
          file: files[0],
          maxSize: MAX_SIZE,
          allowedTypes: imageTypes
        });

        if (checker) {
          setFile(files[0]);
          setFieldValue("idcard_upload", true);

          onClose();
        }
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
  };

  const onCameraDone = () => {
    onCameraClose();
    onClose();
  }

  return (
    <Drawer isOpen={isOpen} onClose={onClose} placement="bottom">
      <DrawerContent bg={"primary.100"} rounded={"3xl"} boxShadow={"xl"}>
        <DrawerBody>
          <Flex gap={5} py={8}>
            <SelectionContainer onClick={onCameraOpen}>
              <CameraInput
                isOpen={isCameraOpen}
                onClose={onCameraDone}
                setFile={setFile}
              />
              <Image src="/assets/registration/camera.png" alt="camera" w={14} />
              <Text fontSize={"sm"} color={"neutral.600"} fontWeight={'bold'}>
                Take a photo
              </Text>
            </SelectionContainer>

            <SelectionContainer onClick={() => refUpload.current?.click()}>
              <Input
                type="file"
                accept="image/*"
                onChange={onUploadFile}
                display={"none"}
                ref={refUpload}
              />
              <Image src="/assets/registration/upload.png" alt="upload" w={14} />
              <Text fontSize={"sm"} color={"neutral.600"} fontWeight={'bold'}>
                Upload file
              </Text>
            </SelectionContainer>
          </Flex>
        </DrawerBody>
      </DrawerContent>
    </Drawer>
  );
}

interface IDCardInputProps {
  file: File | null;
  setFile: (file: File | null) => void;
}

export default function IDCardInput({
  file, setFile
}: IDCardInputProps) {
  const { setTouched, } = useFormikContext<PatientRegister>();
  const { isOpen, onOpen, onClose } = useDisclosure();

  return (
    <Flex
      w={'full'}
      bg={"neutral.100"}
      justify={'center'}
      align={'center'}
      direction={'column'}
      py={file ? 0 : 16}
      rounded={"xl"}
      gap={2}
      _hover={{ bg: "neutral.200" }}
      onClick={() => {
        setTouched({ idcard_upload: true });
        onOpen();
      }}
    >
      <IDCardSelection
        isOpen={isOpen}
        onClose={onClose}
        setFile={setFile}
      />

      {file ? (
        <Flex pos={'relative'} justify={'center'} align={'center'}>
          <Image src={URL.createObjectURL(file)} alt="ID Card" rounded={"xl"} />
          <Text
            fontSize={"sm"}
            color={"neutral.600"}
            fontWeight={'bold'}
            pos={'absolute'}
            bg={'rgba(255, 255, 255, 0.7)'}
            p={3}
            rounded={"xl"}
          >
            Click to change ID Card
          </Text>
        </Flex>
      ) : (
        <FormControl isRequired>
          <Flex align={'center'} justify={'center'} direction={'column'}>
            <Image src="/assets/registration/idcard.png" alt="ID Card" />
            <FormLabel fontSize={"md"} color={"neutral.600"} fontWeight={'bold'}>
              Upload your ID Card
            </FormLabel>
          </Flex>
        </FormControl>
      )}
    </Flex>
  )
}