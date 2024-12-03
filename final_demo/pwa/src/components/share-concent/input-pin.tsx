import { Flex, Modal, ModalBody, ModalContent, ModalFooter, ModalOverlay, Text } from "@chakra-ui/react";
import InputPIN from "@/components/pin/input";
import SubmitPinButton from "../pin/submit/button";

interface ModalInputPinProps {
  isOpen: boolean;
  onClose: () => void;
  onSuccess: () => void;
}

export default function ModalInputPin({
  isOpen,
  onClose,
  onSuccess,
}: ModalInputPinProps) {
  return (
    <Modal isOpen={isOpen} onClose={onClose} size={'full'}>
      <ModalOverlay />
      <ModalContent maxW={'xl'}>
        <ModalBody display={'flex'} flex={1}>
          <Flex
            w={'full'}
            direction={'column'}
            flex={1}
            justify={'space-between'}
            p={5}
          >
            <Flex direction="column" align="center" gap={10} pt={5}>
              <Flex direction={'column'} gap={1}>
                <Text fontSize={"2xl"} textAlign="center" color="neutral.700" fontWeight={'bold'}>
                  Enter Your Login Pin
                </Text>
                <Text fontSize="lg" textAlign="center" color="neutral.700">
                  Please Enter your pin to continue
                </Text>
              </Flex>
              <InputPIN />
            </Flex>
          </Flex>
        </ModalBody>
        <ModalFooter>
          <Flex direction={'column'} w={'full'}>
            <SubmitPinButton
              onClose={onClose}
              onSuccess={onSuccess}
            />
          </Flex>
        </ModalFooter>
      </ModalContent>
    </Modal>
  )
}