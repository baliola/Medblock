"use client"

import { 
  Button,
  Icon,
  Text, 
  useDisclosure,
  Flex,
  Modal,
  ModalContent,
  ModalOverlay,
  Image
} from "@chakra-ui/react";
import { BiDotsVerticalRounded } from "react-icons/bi";

export default function MemberOptionModal() {
  const { isOpen, onOpen, onClose } = useDisclosure();
  
  return (
    <>
      <Button 
        ml={"auto"}
        width={"fit-content"}
        minWidth={0}
        p={0}
        onClick={onOpen}
      >
        <Icon 
          as={BiDotsVerticalRounded} 
          boxSize={6}
          color={'rgba(136, 136, 136, 1)'} 
        />
      </Button>
      <Modal
        isOpen={isOpen}
        onClose={onClose}
        size={{ base: 'full' }}
      >
        <ModalOverlay 
          onClick={onClose}
          background={"transparent"}
        />
        <ModalContent
          border={"none"}
          shadow={"none"}
          marginX={0}
          marginY={"auto"}
          w={"full"}
          h={"full"}
          display={"flex"}
          onClick={onClose}
          background={"transparent"}
        >
          <Flex 
            marginTop={"auto"}
            flexDirection={"column"}
            rowGap={6}
            h={"fit"}
            py={12}
            width={"full"}
            background={"rgba(230, 238, 252, 1)"}
            px={8}
            onClick={(e) => e.stopPropagation()}
          >
            <Button
              display={"flex"}
              alignItems={"center"}
              justifyContent={"start"}
              background={"transparent"}
              gap={3}
            >
              <Image 
                src={"/assets/group-member/share.svg"} 
                alt={"Share"}
              />
              <Text>Share Consent Code</Text>
            </Button>
            <Button
              display={"flex"}
              alignItems={"center"}
              justifyContent={"start"}
              background={"transparent"}
              gap={3}
            >
              <Image 
                src={"/assets/group-member/info.svg"} 
                alt={"Information"}
              />
              <Text>See Account Information</Text>
            </Button>
            <Button
              display={"flex"}
              alignItems={"center"}
              justifyContent={"start"}
              background={"transparent"}
              gap={3}
            >
              <Image 
                src={"/assets/group-member/logout.svg"} 
                alt={"Log Out"}
              />
              <Text>Leave Group</Text>
            </Button>
          </Flex>
        </ModalContent>
      </Modal>
    </>
  )
}