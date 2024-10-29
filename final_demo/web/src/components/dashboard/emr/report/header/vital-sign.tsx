import { Flex, Icon, Text } from "@chakra-ui/react";
import { IconType } from "react-icons/lib";
import { emrDetailReports } from "@/constants/contents/dashboard/emr/detail/reports";

const VitalSignFrame = ({
  label, value, icon
}: {
  label: string;
  value: string;
  icon: IconType;
}) => {
  return (
    <Flex
      direction={"column"}
      border={"2px"}
      borderColor={"primary.700"}
      p={5} gap={2}
      rounded={"xl"}
      w={"full"}
      transition={"all .3s"}
      _hover={{ bg: "primary.200" }}
    >
      <Text color={"neutral.600"} fontWeight={'medium'} fontSize={'sm'}>
        {label}
      </Text>
      <Flex align={'center'} gap={3}>
        <Icon as={icon} boxSize={5} color={"primary.700"} />
        <Text color={"primary.800"} fontWeight={'bold'} fontSize={"lg"}>
          {value}
        </Text>
      </Flex>
    </Flex>
  )
}

interface VitalSignProps {
  vitalSign: {
    blood_pressure: string;
    heart_rate: string;
    respiration: string;
    temperature: string;
    oxygen_saturation: string;
  }
}

export default function VitalSigns({ vitalSign }: VitalSignProps) {
  const sign = emrDetailReports.vital_signs;

  return (
    <Flex direction={'column'} gap={5}>
      <Text fontSize={{ base: 'md', lg: "lg" }} fontWeight={'bold'} color={'neutral.700'}>
        {emrDetailReports.header.vital_sign.title}
      </Text>
      <Flex gap={5}>
        <VitalSignFrame
          label={sign.blood_pressure.label}
          value={`${vitalSign.blood_pressure} ${sign.blood_pressure.unit}`}
          icon={sign.blood_pressure.icon}
        />
        <VitalSignFrame
          label={sign.temperature.label}
          value={`${vitalSign.temperature} ${sign.temperature.unit}`}
          icon={sign.temperature.icon}
        />
      </Flex>
      <Flex gap={5}>
        <VitalSignFrame
          label={sign.heart_rate.label}
          value={`${vitalSign.heart_rate} ${sign.heart_rate.unit}`}
          icon={sign.heart_rate.icon}
        />
        <VitalSignFrame
          label={sign.respiration.label}
          value={`${vitalSign.respiration} ${sign.respiration.unit}`}
          icon={sign.respiration.icon}
        />
        <VitalSignFrame
          label={sign.oxygen_saturation.label}
          value={`${vitalSign.oxygen_saturation} ${sign.oxygen_saturation.unit}`}
          icon={sign.oxygen_saturation.icon}
        />
      </Flex>
    </Flex>
  )
}