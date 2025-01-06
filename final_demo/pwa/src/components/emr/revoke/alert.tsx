import { emrRevokeConfirmation } from "@/constants/contents/emr/revoke/alert";
import { Button, Modal, ModalBody, ModalContent, ModalFooter, ModalHeader, ModalOverlay } from "@chakra-ui/react";

interface RevokeConfirmationAlertProps {
  isOpen: boolean;
  onClose: () => void;
  onConfirm: () => void;
  isLoading: boolean;
}

export default function RevokeConfirmationAlert({
  isOpen, onClose, onConfirm, isLoading
}: RevokeConfirmationAlertProps) {
  const { title, description, button } = emrRevokeConfirmation;

  return (
    <Modal isOpen={isOpen} onClose={onClose} isCentered={true} size={'sm'}>
      <ModalOverlay />
      <ModalContent rounded={"xl"}>
        <ModalHeader textAlign={'center'}>
          {title}
        </ModalHeader>
        <ModalBody textAlign={'center'}>
          {description}
        </ModalBody>
        <ModalFooter gap={3} pt={8}>
          <Button variant={'outline'} w={'full'} onClick={onClose} isLoading={isLoading}>
            {button.cancel.label}
          </Button>
          <Button colorScheme="red" w={'full'} onClick={onConfirm} isLoading={isLoading}>
            {button.confirmation.label}
          </Button>
        </ModalFooter>
      </ModalContent>
    </Modal>
  )
}