"use client"

import { useRouter } from "next/navigation";

import { Button, Flex, Icon, Image, Text, useDisclosure } from "@chakra-ui/react";
// import { signOut } from "next-auth/react";
import { FiLogOut } from "react-icons/fi";

import { FaChevronLeft, FaChevronRight } from "react-icons/fa";
import { LOGO_ONLY, LOGO_WITH_TEXT } from "@/constants/logo";
import { Active, sidebarLinks } from "@/constants/contents/sidebar";
import useMedblockAuth from "@/hooks/useAuth";

export default function Sidebar({
  activeLink
}: {
  activeLink: Active
}) {
  const router = useRouter();
  const {
    isOpen,
    onOpen,
    onClose
  } = useDisclosure({ defaultIsOpen: false });

  const {
    onLogout
  } = useMedblockAuth();

  // const onLogout = async () => {
  //   await signOut({
  //     callbackUrl: "/auth/login",
  //     redirect: true,
  //   });
  // }

  return (
    <Flex
      direction={'column'}
      py={8}
      px={isOpen ? 8 : 4}
      gap={8}
      w={isOpen ? "xs" : "6rem"}
      align={isOpen ? 'start' : 'center'}
      transition="width 0.3s"
      h={'100dvh'}
      pos={'sticky'}
      top={0}
      bg={'primary.100'}
      zIndex={999}
    >
      <Flex
        justify={isOpen ? 'space-between' : 'center'}
        align={'center'}
        w={'full'}
      >
        {isOpen
          ? <Image src={LOGO_WITH_TEXT} alt="logo" mx={'auto'} />
          : <Image src={LOGO_ONLY} alt="logo" mx={'auto'} />
        }
        <Button
          size={'xs'}
          bg={'primary.700'}
          colorScheme="primary"
          color={'white'}
          rounded={'full'}
          p={0}
          pos={'absolute'}
          right={-3}
          top={isOpen ? 28 : 20}
          onClick={isOpen ? onClose : onOpen}
        >
          <Icon
            as={isOpen ? FaChevronLeft : FaChevronRight}
            boxSize={3}
            px={0.5}
          />
        </Button>
      </Flex>

      <Flex direction={"column"} gap={8} flex={1}>
        {sidebarLinks.map((link) => (
          <Flex key={link.name}
            align={'center'}
            gap={4}
            cursor={'pointer'}
            onClick={() => router.push(link.href)}
          >
            <Icon as={link.icon}
              color={link.active === activeLink ? 'primary.700' : 'neutral.500'}
              boxSize={6}
            />
            {isOpen && (
              <Text
                fontSize={"md"}
                fontWeight={'semibold'}
                color={link.active === activeLink ? 'neutral.950' : 'neutral.500'}
              >
                {link.name}
              </Text>
            )}
          </Flex>
        ))}
      </Flex>

      <Flex direction={"column"} gap={4} align={isOpen ? 'start' : 'center'} w={'full'}>
        <Button
          size={'sm'}
          leftIcon={<Icon as={FiLogOut} boxSize={isOpen ? 4 : 6} />}
          variant={isOpen ? 'outline' : 'ghost'}
          colorScheme="red"
          w={'full'}
          onClick={onLogout}
        >
          {isOpen && (
            "Log Out"
          )}
        </Button>
      </Flex>
    </Flex>
  )
}
