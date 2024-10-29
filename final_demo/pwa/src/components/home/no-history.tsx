"use client"

import { Button, Flex, Icon, Image, Stack, Text } from "@chakra-ui/react";
import { FaRegShareFromSquare } from "react-icons/fa6";
import ShareConcentCode from "../share-concent";
import { shareConcentButton } from "@/constants/contents/home/button";
import { noHistory } from "@/constants/contents/no-history";

export default function EmptyHistoryEMR({ refreshData }: { refreshData: () => void }) {
  const { image, alt, header, button } = noHistory;

  return (
    <Flex
      flex={1}
      direction={'column'}
      gap={5}
      justify={'space-between'}
      h={"full"}
    >
      <Stack align={'center'} pt={5}>
        <Image src={image} alt={alt} w={40} />
        <Stack spacing={3} align={'center'} textAlign={'center'}>
          <Text fontSize={'xl'} fontWeight={'bold'}>
            {header.title}
          </Text>
          <Text fontSize={'md'}>
            {header.description}
          </Text>
        </Stack>
        <Button onClick={refreshData}
          colorScheme="gray"
          color={'primary.700'}
          size={'sm'}
          mt={7}
        >
          {button.label}
        </Button>
      </Stack>

      <ShareConcentCode
        leftIcon={
          <Icon as={FaRegShareFromSquare} boxSize={5} />
        }
      >
        {shareConcentButton.label}
      </ShareConcentCode>
    </Flex>
  )
}