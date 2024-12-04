import { assets } from "@/constants/assets"
import { Box, Flex, Text } from "@chakra-ui/react"

export default function AuthLayout({
  children
}: {
  children: React.ReactNode
}) {
  return (
    <Flex
      w={"full"}
      minH={"100dvh"}
      align={'center'}
      justify={{ base: "center", lg: 'end' }}
      bg={"primary.50"}
    >
      <Flex
        direction={'column'}
        pos={'absolute'}
        gap={8}
        top={{ md: 0, lg: 'unset' }}
        left={{ md: 0, lg: 'unset' }}
      >
        <Box
          w={{ base: "50vw", lg: "95vw" }}
          h={{ base: "40dvh", lg: "60dvh" }}
          bgImage={`url(${assets.labs_illustration})`}
          bgRepeat={'no-repeat'}
          bgSize={'contain'}
          zIndex={0}
        />
        <Text
          fontSize={"5xl"}
          w={"fit-content"}
          fontWeight={'bold'}
          textAlign={'center'}
          mx={24}
          lineHeight={1.1}
          whiteSpace={'nowrap'}
          display={{ base: 'none', lg: 'block' }}
        >
          Your {' '}
          <Text as="span" color="accent.700">
            Medical
          </Text>
          {' '} <br />
          Record On Your <br />
          Own {' '}
          <Text as="span" color="primary.600">
            Block
          </Text>
          {' '}
        </Text>
      </Flex>
      <Box
        w={"20vw"}
        h={"30dvh"}
        bgImage={`url(${assets.hospital_illustration})`}
        bgRepeat={'no-repeat'}
        bgSize={'contain'}
        pos={'absolute'}
        top={{ md: 0, lg: 10 }}
        right={0}
        zIndex={0}
        display={{ base: 'none', md: 'block' }}
      />
      <Box
        w={"40vw"}
        h={{ md: "20dvh", lg: "30dvh" }}
        bgImage={`url(${assets.umbrella_illustration})`}
        bgRepeat={'no-repeat'}
        bgSize={'contain'}
        pos={'absolute'}
        bottom={{ md: -20, lg: 24 }}
        right={0}
        zIndex={0}
        display={{ base: 'none', md: 'block' }}
      />

      <Box
        w={{ base: "full", md: "lg", lg: "md" }}
        h={"fit-content"}
        zIndex={1}
        bg="rgba(251, 251, 254, 0.65)"
        me={{ lg: 52 }}
        backdropFilter={"blur(20px)"}
        border={"2px solid"}
        borderColor={"white"}
        rounded={"3xl"}
      >
        {children}
      </Box>
    </Flex>
  )
}