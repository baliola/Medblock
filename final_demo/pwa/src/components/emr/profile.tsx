"use client"

import { useRouter } from "next/navigation"
import { Flex, Icon, Text } from "@chakra-ui/react"
import { IoCloseCircle } from "react-icons/io5"

import HorizontalProfile from "@/components/profile/horizontal"
import { emrProfileCloseAccess } from "@/constants/contents/emr/detail/button"

const RevokeAccess = () => {
  const router = useRouter();

  return (
    <Flex
      direction={'column'}
      gap={2}
      align={'center'}
      justify={'center'}
      w={"fit-content"}
      bg={'accent.200'}
      rounded={'lg'}
      p={2}
      cursor={'pointer'}
      _hover={{ bg: 'accent.300' }}
      onClick={() => router.push(emrProfileCloseAccess.redirect)}
    >
      <Icon as={IoCloseCircle} boxSize={5} color={'accent.600'} />
      <Text fontSize={'xs'} color={'accent.600'} fontWeight={'bold'}>
        {emrProfileCloseAccess.label}
      </Text>
    </Flex>
  )
}

export default function EMRProfile() {
  return (
    <HorizontalProfile
      rightIcon={
        <RevokeAccess />
      }
    />
  )
}