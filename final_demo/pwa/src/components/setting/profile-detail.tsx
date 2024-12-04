"use client"

import { Flex, Icon, Text } from "@chakra-ui/react";
import { useProfileStore } from "@/store/profile-store";
import { settingProfile } from "@/constants/contents/setting/profile";

interface InfoRowProps {
  icon: React.ElementType;
  title: string;
  value: string;
}

const parsingDate = (date: string) => {
  const splitDate = date.split('T')[0];
  const newDate = new Date(splitDate);
  return newDate.toLocaleDateString('id-ID', {
    day: 'numeric',
    month: 'long',
    year: 'numeric'
  });
}

const InfoRow = ({
  icon,
  title,
  value
}: InfoRowProps) => {
  return (
    <Flex align={'center'} gap={4}>
      <Icon as={icon} boxSize={5} color={"neutral.700"} />
      <Flex direction={'column'} color={"neutral.700"}>
        <Text fontSize={'sm'}>{title}</Text>
        <Text fontSize={'md'} fontWeight={'bold'}>{value}</Text>
      </Flex>
    </Flex>
  );
}

export default function SettingDetailProfile() {
  const patient = useProfileStore(state => state.profile);
  const { contents, title } = settingProfile;

  return (
    <Flex direction={"column"} gap={3} flex={1}>
      <Text fontSize={'md'} fontWeight={'bold'} color={"neutral.700"}>
        {title}
      </Text>
      <Flex gap={3} direction={'column'}>
        <InfoRow
          icon={contents.home_address.icon}
          title={contents.home_address.label}
          value={patient?.patient.V1.address || ""}
        />
        <InfoRow
          icon={contents.martial_status.icon}
          title={contents.martial_status.label}
          value={patient?.patient.V1.martial_status || ""}
        />
        <InfoRow
          icon={contents.birthdate.icon}
          title={contents.birthdate.label}
          value={`
              ${patient?.patient.V1.place_of_birth}, 
              ${parsingDate(patient?.patient.V1.date_of_birth || "")}
          `}
        />
      </Flex>
    </Flex>
  )
}