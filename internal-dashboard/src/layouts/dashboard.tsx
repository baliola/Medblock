import Sidebar from "@/components/sidebar";
import { Active } from "@/constants/contents/sidebar";
import { Flex } from "@chakra-ui/react";

export default function DashboardLayout({
  activeLink,
  children
}: {
  activeLink: Active,
  children: React.ReactNode
}) {
  return (
    <Flex bg={'neutral.50'} w={'full'} minH={'100dvh'}>
      <Sidebar activeLink={activeLink} />
      <Flex
        flex={1}
        flexDirection={'column'}
        w={'full'}
      >
        {children}
      </Flex>
    </Flex>
  )
}