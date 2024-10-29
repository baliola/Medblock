"use client"

import { settingBackNavigation } from "@/constants/contents/setting/back-navigation";
import { Flex, Icon, Text } from "@chakra-ui/react";
import { useRouter } from "next/navigation";

export default function SettingBackNavigation() {
  const router = useRouter();
  const { label, icon, redirect } = settingBackNavigation;

  return (
    <Flex
      align={'center'}
      w={'fit-content'}
      gap={3}
      onClick={() => router.push(redirect)}
    >
      <Icon as={icon} boxSize={5} color={'neutral.500'} />
      <Text
        fontSize={'lg'}
        fontWeight={'bold'}
      >
        {label}
      </Text>
    </Flex>
  )
}