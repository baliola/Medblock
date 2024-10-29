import { GetPatientInfoResponse } from "@/declarations/patient_registry/patient_registry.did";
import { calculateAge } from "@/utils/calculate-age";
import { Avatar, Flex, Icon, Tag, Text } from "@chakra-ui/react";
import { IoIosFemale, IoIosMale } from "react-icons/io";

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
    size={{ base: "sm" }}
    loading="eager"
  />
)

export default function HorizontalProfile({
  profile
}: {
  profile: GetPatientInfoResponse
}) {
  return (
    <Flex
      bg={'white'}
      rounded={'lg'}
      p={3}
      gap={5}
      w={'full'}
    >
      <ProfileAvatar
        avatar={profile.patient.V1.name}
        name={profile.patient.V1.name}
      />

      <Flex direction={'column'} gap={1} color={"neutral.700"} w={"full"}>
        <Text fontSize={{ base: 'sm' }} fontWeight={'bold'}>
          {profile.patient.V1.name}
        </Text>

        <Flex
          align={'center'}
          justify={'space-between'}
          gap={1}
          w={"full"}
          flexWrap={'wrap'}
        >
          <Flex align={'center'} gap={2} w={'fit-content'}>
            {profile.patient.V1.gender === 'male'
              ? <Icon as={IoIosMale} color={"blue.500"} boxSize={4} />
              : <Icon as={IoIosFemale} color={"pink.500"} boxSize={4} />
            }

            <Text fontSize={{ base: 'xs' }} whiteSpace={'nowrap'}>
              {calculateAge(profile.patient.V1.date_of_birth)}
            </Text>
          </Flex>

          <Tag fontSize={'xs'} whiteSpace={'nowrap'}>
            ID: {profile.nik.slice(0, 10) + "..."}
          </Tag>
        </Flex>
      </Flex>
    </Flex>
  )
}