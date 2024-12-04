"use client"

import { Divider, Flex, Icon, Text } from "@chakra-ui/react";
import { FaHospital } from "react-icons/fa6";
import { MdMonetizationOn } from "react-icons/md";
import EMRHospitalInfo from "./hospital-info";
import { useEMRStore } from "@/store/emr-store";
import { emrDetailHeader } from "@/constants/contents/emr/detail/header";

export default function EMRHeader() {
  const emr = useEMRStore((state) => state.emr);
  if (!emr) return null;

  const emrData = emr.body.reduce((acc, item) => {
    acc[item.key] = item.value;
    return acc;
  }, {} as Record<string, string>);

  const HeaderSection = ({ keys, title }: { keys: string; title: string }) => {
    const value = emrData[keys] || '';
    if (!value) return null;
    return (
      <Flex flex={1} direction={'column'}>
        <Text fontSize={'xs'}>{title}</Text>
        <Text fontSize={'md'} fontWeight={'bold'}>
          {value ?? "-"}
        </Text>
      </Flex>
    );
  };

  return (
    <Flex direction={'column'} gap={5}>
      <Flex w={'full'} color={'neutral.700'}>
        {emrDetailHeader.report.map((item, index) => (
          <HeaderSection key={index}
            title={item.title}
            keys={item.key}
          />
        ))}
      </Flex>
      <Divider />
      <EMRHospitalInfo />
      {emrDetailHeader.information.map((info, index) => (
        <Flex key={index} align={'center'} p={4} bg={info.bgColor} rounded={"2xl"} gap={4}>
          <Icon as={info.icon} boxSize={5} color={info.textColor} />
          <Text fontSize={"md"} fontWeight={"bold"} color={info.textColor}>
            {info.title}
          </Text>
        </Flex>
      ))}
    </Flex>
  )
}