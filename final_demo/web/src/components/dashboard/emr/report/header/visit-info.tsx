import { Flex, Text } from "@chakra-ui/react";

export default function VisitInfo({
  title, value
}: {
  title: string;
  value: string;
}) {
  return (
    <Flex direction={'column'} gap={1} w={'full'} >
      <Text fontSize={'xs'}>{title}</Text>
      <Text fontSize={'md'} fontWeight={'bold'}>
        {value}
      </Text>
    </Flex>
  )
}