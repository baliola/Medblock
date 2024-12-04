"use client";

import { Flex, Text } from "@chakra-ui/react";
import { useAuthState } from "@ic-reactor/react";
import dynamic from "next/dynamic";
import { adminHeader } from "@/constants/contents/admin/header";
import AddAdminModal from "./add/admin";

const NFIDButtonLogin = dynamic(() => import("@/components/auth/nfid/index"), {
  ssr: false,
});

const Header = () => (
  <Text fontSize={"2xl"} fontWeight={"bold"} w={"full"}>
    {adminHeader.title}
  </Text>
);

export default function AdminPageContent() {
  const { authenticated } = useAuthState();

  return (
    <Flex w={"full"} flex={1}>
      <Flex w={"full"} direction={"column"} p={10} gap={8}>
        <Flex justifyContent={"space-between"} alignItems={"center"} flexDirection={"column"} columnGap={8}>
          <Header />
          <Flex justifyContent={"end"} w={"full"}>
            <Flex w={"fit"}>
              {authenticated ? <AddAdminModal /> : <NFIDButtonLogin />}
            </Flex>
          </Flex>
        </Flex>
        {/* <HAMData /> */}
      </Flex>
    </Flex>
  );
}
