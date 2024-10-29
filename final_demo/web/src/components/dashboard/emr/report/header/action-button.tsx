import { emrDetailHeader } from "@/constants/contents/dashboard/emr/detail/header";
import { Flex, Icon, Text } from "@chakra-ui/react";

export default function ActionButtons() {
  return (
    <Flex gap={5}>
      {emrDetailHeader.information.map((info, index) => (
        <Flex key={index} align={'center'} p={4} bg={info.bgColor} rounded={"2xl"} gap={4}>
          <Icon as={info.icon} boxSize={5} color={info.textColor} />
          <Text fontSize={"sm"} fontWeight={"bold"} color={info.textColor}>
            {info.title}
          </Text>
        </Flex>
      ))}
    </Flex>
  )
}