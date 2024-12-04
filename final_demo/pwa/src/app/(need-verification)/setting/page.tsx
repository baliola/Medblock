"use client"

import dynamic from "next/dynamic";
import { Divider, Flex, Stack } from "@chakra-ui/react";

import VerticalProfile from "@/components/profile/vertical";
import SettingBackNavigation from "@/components/setting/back-navigation";
import ChangePinSetting from "@/components/setting/change-pin";
import NotificationSetting from "@/components/setting/notification";
import SettingDetailProfile from "@/components/setting/profile-detail";

const SettingSignOut = dynamic(
  () => import("@/components/setting/signout"), {
  ssr: false
});

export default function SettingPage() {
  return (
    <Flex
      flex={1}
      gap={5}
      direction={'column'}
      w={'full'}
      justify={'space-between'}
    >
      <Stack spacing={5} divider={<Divider />}>
        <Flex direction={'column'} w={'full'} gap={3}>
          <SettingBackNavigation />
          <Flex align={'center'} justify={'center'} w={'full'}>
            <VerticalProfile />
          </Flex>
        </Flex>
        <SettingDetailProfile />
        <NotificationSetting />
        <ChangePinSetting />
      </Stack>
      <SettingSignOut />
    </Flex>
  )
}