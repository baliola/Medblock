"use client"

import { bottomBarLinks } from "@/constants/contents/bottom-bar";
import { Flex, Icon, Text } from "@chakra-ui/react"
import { useRouter } from "next/navigation";

export type Active = "home" | "emr" | "group" | "setting" | "insurance";

export default function BottomBar({
  active
}: {
  active: Active
}) {
  const router = useRouter();

  return (
    <Flex w={"full"} align={'end'}>
      {bottomBarLinks.map((link) => (
        <Flex
          key={link.name}
          direction="column"
          align="center"
          justify="center"
          gap={1}
          flex={1}
          color={
            link.active === active
              ? "primary.500"
              : "neutral.400"
          }
          onClick={() => router.push(link.href)}
        >
          <Icon
            as={link.icon}
            boxSize={link.active === active ? 6 : 5}
          />
          <Text fontSize={"xs"}>
            {link.name}
          </Text>
        </Flex>
      ))}
    </Flex>
  )
}