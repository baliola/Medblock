import { Image, Modal, ModalBody, ModalCloseButton, ModalContent, ModalHeader, ModalOverlay } from "@chakra-ui/react";

interface IDCardViewProps {
  isOpen: boolean;
  onCLose: () => void;
  data: string;
}

export default function IDCardView({
  isOpen, onCLose, data
}: IDCardViewProps) {
  return (
    <Modal isOpen={isOpen} onClose={onCLose} size={"xl"}>
      <ModalOverlay />
      <ModalContent>
        <ModalHeader>ID Card</ModalHeader>
        <ModalCloseButton />
        <ModalBody pb={5}>
          <Image src={data} alt={data} />
        </ModalBody>
      </ModalContent>
    </Modal>
  )
}