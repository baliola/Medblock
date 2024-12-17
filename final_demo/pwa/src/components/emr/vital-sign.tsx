"use client"

import { Flex, Icon, Text } from "@chakra-ui/react";
import { IconType } from "react-icons/lib";
import { emrDetailReports } from "@/constants/contents/emr/detail/reports";

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
      p={5} py={3} gap={1}
      rounded={"2xl"}
      w={"full"}
      transition={"all .3s"}
      _hover={{ bg: "primary.200" }
      }
    >
      <Text fontSize={'xs'} color={"neutral.600"} fontWeight={'medium'}>
        {label}
      </Text>
      <Flex align={'center'} gap={2}>
        <Icon as={icon} boxSize={6} color={"primary.700"} />
        <Text color={"primary.800"} fontWeight={'bold'} fontSize={"md"}>
          {value}
        </Text>
      </Flex>
    </Flex>
  )
}

interface VitalSignProps {
  vitalSign: {
    blood_pressure: string;
    temperature: string;
    heart_rate: string;
    respiration: string;
    oxygen_saturation: string;
  }
}

export default function EMRVitalSigns({ vitalSign }: VitalSignProps) {
  const sign = emrDetailReports.vital_signs;
  return (
    <Flex direction={'column'} gap={4}>
      <Text fontSize={'lg'} color={"neutral.700"} fontWeight={"bold"} px={1}>
        {emrDetailReports.header.vital_sign.title}
      </Text>
      <VitalSignFrame
        label={sign.blood_pressure.label}
        value={`${vitalSign.blood_pressure} ${sign.blood_pressure.unit}`}
        icon={sign.blood_pressure.icon}
      />
      <Flex gap={4}>
        <VitalSignFrame
          label={sign.temperature.label}
          value={`${vitalSign.temperature} ${sign.temperature.unit}`}
          icon={sign.temperature.icon}
        />
        <VitalSignFrame
          label={sign.heart_rate.label}
          value={`${vitalSign.heart_rate} ${sign.heart_rate.unit}`}
          icon={sign.heart_rate.icon}
        />
      </Flex>
      <Flex gap={4}>
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