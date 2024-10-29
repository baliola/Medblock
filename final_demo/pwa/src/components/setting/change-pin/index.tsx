"use client"

import { useRouter } from "next/navigation";
import { Flex, Icon, Text } from "@chakra-ui/react";
import { settingButton } from "@/constants/contents/setting/button";

export default function ChangePinSetting() {
  const router = useRouter();
  const { change_pin } = settingButton;

  return (
    <Flex
      justify={'space-between'}
      align={'center'}
      cursor={'pointer'}
      onClick={() => router.push(change_pin.redirect)}
    >
      <Flex align={'center'} gap={4}>
        <Icon
          as={change_pin.leftIcon}
          color={'primary.500'}
          boxSize={5}
        />
        <Text fontWeight={'bold'}>
          {change_pin.label}
        </Text>
      </Flex>
      <Icon
        as={change_pin.rightIcon}
        color={'gray.400'}
        boxSize={4}
      />
    </Flex>
  )
}