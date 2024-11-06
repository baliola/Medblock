import { Profile } from "@/constants/contents/uam/detail";
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
  profile: Profile
}) {
  return (
    <Flex
      bg={'white'}
      rounded={'2xl'}
      px={3} py={5}
      gap={5}
      w={'full'}
    >
      <ProfileAvatar
        avatar={profile.fullName}
        name={profile.fullName}
      />

      <Flex direction={'column'} gap={1} color={"neutral.700"} w={"full"}>
        <Text fontSize={'lg'} fontWeight={'bold'}>
          {profile.fullName}
        </Text>

        <Flex align={'center'} justify={'space-between'} gap={3} w={"full"}>
          <Flex align={'center'} gap={2}>
            {profile.gender === 'male'
              ? <Icon as={IoIosMale} color={"blue.500"} boxSize={5} />
              : <Icon as={IoIosFemale} color={"pink.500"} boxSize={5} />
            }

            <Text fontWeight={'semibold'} fontSize={'sm'}>
              {calculateAge(profile.dateBirth)}
            </Text>
          </Flex>

          <Tag fontSize={'sm'}>
            ID: {profile?.nik.slice(0, 10) + '...'}
          </Tag>
        </Flex>
      </Flex>
    </Flex>
  )
}