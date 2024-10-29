"use client"

import { settingButton } from "@/constants/contents/setting/button";
import { Flex, Icon, Text } from "@chakra-ui/react";

export default function NotificationSetting() {
  const { notification } = settingButton;
  return (
    <Flex justify={'space-between'} align={'center'}>
      <Flex align={'center'} gap={4}>
        <Icon
          as={notification.leftIcon}
          color={'warning.600'}
          boxSize={5}
        />
        <Text fontWeight={'bold'}>
          {notification.label}
        </Text>
      </Flex>
      <Icon
        as={notification.rightIcon}
        color={'gray.400'}
        boxSize={4}
      />
    </Flex>
  )
}