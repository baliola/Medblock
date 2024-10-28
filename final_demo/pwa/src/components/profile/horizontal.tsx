"use client"

import { patientCanisterId } from "@/config/canisters/patient.canister";
import { GetPatientInfoResponse } from "@/declarations/patient_registry/patient_registry.did";
import { PatientActor, usePatientQuery } from "@/services/patients";
import { calculateAge } from "@/utils/calculate-age";
import { Avatar, Flex, Icon, Skeleton, Text } from "@chakra-ui/react";
import { useState } from "react";
import { IoIosFemale, IoIosMale } from "react-icons/io";

const ProfileAvatar = ({
  avatar,
  name
}: {
  avatar?: string | undefined;
  name: string;
}) => (
  <Avatar
    src={avatar}
    name={name}
    borderRadius={10}
    size={'lg'}
    loading="eager"
  />
)

const UserProfile = ({
  profile
}: {
  profile: GetPatientInfoResponse
}) => {
  return (
    <Flex
      gap={5}
      align={'center'}
    >
      <ProfileAvatar
        name={profile?.patient.V1.name}
      />

      <Flex direction={'column'} color={"neutral.700"}>
        <Text fontSize={'lg'} fontWeight={'bold'}>
          {profile?.patient.V1.name}
        </Text>

        <Flex align={'center'} gap={3}>
          {profile?.patient.V1.gender.toLowerCase() === 'male'
            ? <Icon as={IoIosMale} color={"blue.500"} boxSize={4} />
            : <Icon as={IoIosFemale} color={"pink.500"} boxSize={4} />
          }

          <Text fontWeight={'semibold'} fontSize={'sm'}>
            {calculateAge(profile?.patient.V1.date_of_birth)}
          </Text>
        </Flex>
      </Flex>
    </Flex>
  )
}

const Profile = ({ rightIcon }: { rightIcon?: React.ReactNode }) => {
  const [patientData, setPatientData] = useState<GetPatientInfoResponse>();

  const {
    loading: patientLoading,
  } = usePatientQuery({
    functionName: "get_patient_info",
    refetchOnMount: true,
    onSuccess(data) {
      setPatientData(data)
    },
  });

  return (
    <Flex justify={'space-between'} align={'center'}>
      {patientLoading
        ? <Skeleton w={'full'} h={20} rounded={"xl"} />
        : <UserProfile profile={patientData as any} />
      }
      {rightIcon}
    </Flex>
  )
}

export default function HorizontalProfile({
  rightIcon
}: {
  rightIcon?: React.ReactNode
}) {
  return (
    <PatientActor
      canisterId={patientCanisterId}
    >
      <Profile rightIcon={rightIcon} />
    </PatientActor>
  )
}