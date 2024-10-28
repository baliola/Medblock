"use client"

import { patientCanisterId } from "@/config/canisters/patient.canister";
import { PatientActor, usePatientQuery } from "@/services/patients";
import { useProfileStore } from "@/store/profile-store";
import { calculateAge } from "@/utils/calculate-age";
import { Avatar, Flex, Icon, Skeleton, Tag, Text } from "@chakra-ui/react";
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

const UserProfile = () => {
  const profile = useProfileStore(state => state.profile);
  return (
    <Flex
      gap={5}
      align={'center'}
      direction={'column'}
    >
      <ProfileAvatar
        name={profile?.patient.V1.name || ""}
      />

      <Flex direction={'column'} align={'center'} color={"neutral.700"} gap={1}>
        <Text fontSize={'lg'} fontWeight={'bold'}>
          {profile?.patient.V1.name}
        </Text>

        <Flex align={'center'} gap={3}>
          {profile?.patient.V1.gender.toLowerCase() === 'male'
            ? <Icon as={IoIosMale} color={"blue.500"} boxSize={4} />
            : <Icon as={IoIosFemale} color={"pink.500"} boxSize={4} />
          }

          <Text fontWeight={'semibold'} fontSize={'sm'}>
            {calculateAge(profile?.patient.V1.date_of_birth || "")}
          </Text>
          <Text fontWeight={'semibold'} fontSize={'sm'}>
            {profile?.patient.V1.martial_status}
          </Text>
        </Flex>
        <Tag fontSize={'sm'}>
          ID: {profile?.nik.slice(0, 20) + "..."}
        </Tag>
      </Flex>
    </Flex>
  )
}

const Profile = ({ rightIcon }: { rightIcon?: React.ReactNode }) => {
  const setProfile = useProfileStore(state => state.setProfile);

  const {
    loading: patientLoading,
  } = usePatientQuery({
    functionName: "get_patient_info",
    refetchOnMount: true,
    onSuccess(data) {
      setProfile(data as any);
    },
  });

  return (
    <Flex justify={'space-between'} align={'center'}>
      {patientLoading
        ? <Skeleton w={'full'} h={20} rounded={"xl"} />
        : <UserProfile />
      }
      {rightIcon}
    </Flex>
  )
}

export default function VerticalProfile({
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