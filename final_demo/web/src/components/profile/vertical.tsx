import { GetPatientInfoResponse } from "@/declarations/patient_registry/patient_registry.did";
import { calculateAge } from "@/utils/calculate-age";
import { Avatar, Flex, Icon, Tag, Text } from "@chakra-ui/react";
import { IoIosFemale, IoIosInformationCircleOutline, IoIosMale } from "react-icons/io";

const ProfileAvatar = ({
  avatar,
  name
}: {
  avatar: string;
  name: string;
}) => (
  <Avatar
    src={avatar}
    name={name}
    rounded={'md'}
    size={'md'}
    loading="eager"
  />
);

const ProfileHeader = () => (
  <Flex justify={'space-between'} w={'full'} px={5}>
    <Text fontWeight={'medium'}>
      Profile
    </Text>
    <Icon
      as={IoIosInformationCircleOutline}
      boxSize={5}
      color={"primary.400"}
    />
  </Flex>
)

export default function VerticalProfile({
  profile
}: {
  profile: GetPatientInfoResponse
}) {
  return (
    <Flex
      bg={'primary.100'}
      rounded={'xl'}
      py={5}
      gap={8}
      direction={'column'}
      align={'center'}
    >
      <ProfileHeader />
      <Flex
        direction={'column'}
        gap={8}
        align={'center'}
        justify={'center'}
        px={10}
      >
        <ProfileAvatar
          avatar={profile?.patient.V1.name}
          name={profile?.patient.V1.name}
        />

        <Flex direction={'column'} gap={2} color={"neutral.700"} align={'center'}>
          <Flex align={'center'} gap={2} w={'fit-content'}>
            {profile.patient.V1.gender === 'male'
              ? <Icon as={IoIosMale} color={"blue.500"} boxSize={4} />
              : <Icon as={IoIosFemale} color={"pink.500"} boxSize={4} />
            }

            <Text fontSize={{ base: 'xs' }} whiteSpace={'nowrap'}>
              {calculateAge(profile.patient.V1.date_of_birth)}
            </Text>
          </Flex>

          <Text fontSize={{ base: 'sm' }} fontWeight={'bold'} textTransform={"capitalize"}>
            {profile.patient.V1.name}
          </Text>

          <Tag fontSize={'xs'} whiteSpace={'nowrap'} bg={'gray.200'}>
            ID: {profile.nik.slice(0, 10) + "..."}
          </Tag>
        </Flex>
      </Flex>
    </Flex>
  )
}