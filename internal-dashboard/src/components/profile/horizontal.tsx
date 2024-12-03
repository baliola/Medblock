import { PatientWithNik } from "@/canister/declarations/patient_registry/patient_registry.did";
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
    borderRadius={10}
    size={'lg'}
    loading="eager"
  />
)

export default function HorizontalProfile({
  profile
}: {
  profile: PatientWithNik
}) {
  return (
    <Flex
      bg={'white'}
      rounded={'2xl'}
      px={3} py={3}
      gap={5}
      alignItems={"center"}
      w={'full'}
    >
      <ProfileAvatar
        avatar={profile.info.V1.name}
        name={profile.info.V1.name}
      />

      <Flex direction={'column'} gap={1} color={"neutral.700"} w={"full"}>
        <Text fontSize={'lg'} fontWeight={'bold'} textTransform={"capitalize"}>
          {profile.info.V1.name}
        </Text>

        <Flex  align={'center'} justify={'space-between'} gap={3} w={"full"}>
          <Flex align={'center'} gap={2}>
            {profile.info.V1.gender.toLowerCase() === 'male'
              ? <Icon as={IoIosMale} color={"blue.500"} boxSize={5} />
              : <Icon as={IoIosFemale} color={"pink.500"} boxSize={5} />
            }

            <Text fontWeight={'semibold'} fontSize={'sm'}>
              {calculateAge(profile.info.V1.date_of_birth)}
            </Text>
          </Flex>

          <Tag fontSize={'sm'} title={profile?.nik.slice(0, 10) + '...'}>
            ID: {profile?.nik.slice(0, 10) + '...'}
          </Tag>
        </Flex>
      </Flex>
    </Flex>
  )
}