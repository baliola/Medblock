import AddPinButton from "@/components/pin/add/button";
import InputPIN from "@/components/pin/input";
import { headerAddPin } from "@/constants/contents/pin/header";
import { Flex, Text } from "@chakra-ui/react";

export default function AddPinPage() {
  const { title, description } = headerAddPin;

  return (
    <Flex
      w={'full'}
      direction={'column'}
      flex={1}
      justify={'space-between'}
      p={5}
    >
      <Flex direction="column" align="center" gap={10} pt={5}>
        <Flex direction={'column'} gap={1}>
          <Text fontSize={"2xl"} textAlign="center" color="neutral.700" fontWeight={'bold'}>
            {title}
          </Text>
          <Text fontSize="lg" textAlign="center" color="neutral.700">
            {description}
          </Text>
        </Flex>
        <InputPIN />
      </Flex>
      <AddPinButton />
    </Flex>
  )
}