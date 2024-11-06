import { Flex, Text } from "@chakra-ui/react";
import { uamHeader } from "@/constants/contents/uam/header";
import UAMData from "@/components/uam";
import UAMDetail from "@/components/uam/detail";

interface PageProps {
  searchParams: {
    currentPage?: number;
    limit?: number;
    user?: string;
  }
}

const Header = () => (
  <Text fontSize={'2xl'} fontWeight={'bold'}>
    {uamHeader.title}
  </Text>
)

export default async function UAMPage({ searchParams }: PageProps) {
  const user = searchParams.user || null;

  return (
    <Flex w={'full'} flex={1}>
      <Flex w={'full'} direction={'column'} p={10} gap={8}>
        <Header />
        <UAMData />
      </Flex>

      {user && <UAMDetail />}
    </Flex>
  )
}