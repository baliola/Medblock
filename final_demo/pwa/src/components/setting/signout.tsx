"use client"

import { settingButton } from "@/constants/contents/setting/button";
import useMedblockAuth from "@/hooks/useAuth";
import { Button, Icon } from "@chakra-ui/react";

export default function SettingSignOut() {
  const { onLogout } = useMedblockAuth();
  const { signOut } = settingButton;
  return (
    <Button
      size={'md'}
      leftIcon={<Icon as={signOut.icon} boxSize={4} />}
      colorScheme={signOut.color}
      w={'full'}
      onClick={onLogout}
    >
      {signOut.label}
    </Button>
  )
}